mod cache;
mod database;
mod env;
mod logger;
mod models;
mod routes;
mod schedules;
mod telemetry;

use axum::{
    ServiceExt,
    body::Body,
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
    routing::get,
};
use colored::Colorize;
use sentry_tower::SentryHttpLayer;
use serde_json::json;
use sha2::Digest;
use std::{net::IpAddr, sync::Arc, time::Instant};
use tokio::sync::RwLock;
use tower::Layer;
use tower_http::{
    catch_panic::CatchPanicLayer, cors::CorsLayer, normalize_path::NormalizePathLayer,
};
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa_axum::router::OpenApiRouter;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const GIT_COMMIT: &str = env!("CARGO_GIT_COMMIT");

fn handle_panic(_err: Box<dyn std::any::Any + Send + 'static>) -> Response<Body> {
    logger::log(
        logger::LoggerLevel::Error,
        "a request panic has occurred".bright_red().to_string(),
    );

    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .header("Content-Type", "application/json")
        .body(Body::from(
            routes::ApiError::new(&["internal server error"])
                .to_value()
                .to_string(),
        ))
        .unwrap()
}

async fn handle_request(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let ip = extract_ip(req.headers())
        .map(|ip| ip.to_string())
        .unwrap_or_else(|| "unknown".to_string());

    logger::log(
        logger::LoggerLevel::Info,
        format!(
            "{} {}{} {}",
            format!("HTTP {}", req.method()).green().bold(),
            req.uri().path().cyan(),
            if let Some(query) = req.uri().query() {
                format!("?{}", query)
            } else {
                "".to_string()
            }
            .bright_cyan(),
            format!("({})", ip).bright_black(),
        ),
    );

    Ok(next.run(req).await)
}

async fn handle_etag(req: Request, next: Next) -> Result<Response, StatusCode> {
    let if_none_match = req.headers().get("If-None-Match").cloned();

    let response = next.run(req).await;
    let mut hash = sha2::Sha256::new();

    let (mut parts, body) = response.into_parts();
    let body_bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();

    hash.update(body_bytes.as_ref());
    let hash = format!("{:x}", hash.finalize());

    parts.headers.insert("ETag", hash.parse().unwrap());

    if if_none_match == Some(hash.parse().unwrap()) {
        let mut cached_response = Response::builder()
            .status(StatusCode::NOT_MODIFIED)
            .body(Body::empty())
            .unwrap();

        for (key, value) in parts.headers.iter() {
            cached_response.headers_mut().insert(key, value.clone());
        }

        return Ok(cached_response);
    }

    Ok(Response::from_parts(parts, Body::from(body_bytes)))
}

#[inline]
pub fn extract_ip(headers: &HeaderMap) -> Option<IpAddr> {
    let ip = headers
        .get("x-real-ip")
        .or_else(|| headers.get("x-forwarded-for"))
        .map(|ip| ip.to_str().unwrap_or_default())
        .unwrap_or_default();

    if ip.is_empty() {
        return None;
    }

    let ip = if ip.contains(',') {
        ip.split(',').next().unwrap_or_default().trim().to_string()
    } else {
        ip.to_string()
    };

    ip.parse().ok()
}

#[tokio::main]
async fn main() {
    let env = env::Env::parse();

    let _guard = sentry::init((
        env.sentry_url.clone(),
        sentry::ClientOptions {
            server_name: env.server_name.clone().map(|s| s.into()),
            release: Some(format!("{}:{}", VERSION, GIT_COMMIT).into()),
            traces_sample_rate: 1.0,
            ..Default::default()
        },
    ));

    let env = Arc::new(env);
    let database = Arc::new(database::Database::new(env.clone()).await);
    let cache = Arc::new(cache::Cache::new(env.clone()).await);

    let state = Arc::new(routes::AppState {
        start_time: Instant::now(),
        version: format!("{}:{}", VERSION, GIT_COMMIT),

        github_releases: RwLock::new(Vec::new()),

        database: database.clone(),
        cache: cache.clone(),
        telemetry: telemetry::TelemetryLogger::new(database.clone(), cache.clone(), env.clone()),
        env,
    });

    {
        let state = state.clone();

        tokio::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;

                state.telemetry.process().await.unwrap_or_default();
            }
        });
    }

    tokio::spawn(schedules::github::run(state.clone()));
    if state.env.update_prices && (state.env.sxc_token.is_some() || state.env.bbb_token.is_some()) {
        tokio::spawn(schedules::prices::run(state.clone()));
    }

    let app = OpenApiRouter::new()
        .nest("/api", routes::router(&state))
        .route(
            "/",
            get(|| async move {
                let mut headers = HeaderMap::new();

                headers.insert("Content-Type", "text/html".parse().unwrap());

                (
                    StatusCode::OK,
                    headers,
                    include_str!("../static/index.html"),
                )
            }),
        )
        .route(
            // for compatibility reasons
            "/send/{panel}/{data}",
            get(|| async move { (StatusCode::OK, axum::Json(json!({}))) }),
        )
        .fallback(|| async {
            (
                StatusCode::NOT_FOUND,
                axum::Json(json!({
                    "success": false,
                    "errors": ["route not found"]
                })),
            )
        })
        .layer(CatchPanicLayer::custom(handle_panic))
        .layer(CorsLayer::very_permissive())
        .layer(axum::middleware::from_fn(handle_request))
        .route_layer(axum::middleware::from_fn(handle_etag))
        .route_layer(SentryHttpLayer::with_transaction())
        .with_state(state.clone());

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", &state.env.bind, state.env.port))
        .await
        .unwrap();

    logger::log(
        logger::LoggerLevel::Info,
        format!(
            "{} listening on {} {}",
            "http server".bright_red(),
            listener.local_addr().unwrap().to_string().cyan(),
            format!(
                "(app@{}, {}ms)",
                VERSION,
                state.start_time.elapsed().as_millis()
            )
            .bright_black()
        ),
    );

    let (router, mut openapi) = app.split_for_parts();
    openapi.info.version = state.version.clone();
    openapi.info.description = None;
    openapi.info.title = "Blueprint API".to_string();
    openapi.info.contact = None;
    openapi.info.license = None;
    openapi.servers = Some(vec![utoipa::openapi::Server::new(
        state.env.app_url.clone(),
    )]);
    openapi.components.as_mut().unwrap().add_security_scheme(
        "api_key",
        SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
    );

    let router = router.route("/openapi.json", get(|| async move { axum::Json(openapi) }));

    axum::serve(
        listener,
        ServiceExt::<Request>::into_make_service(
            NormalizePathLayer::trim_trailing_slash().layer(router),
        ),
    )
    .await
    .unwrap();
}
