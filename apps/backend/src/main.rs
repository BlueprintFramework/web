use crate::routes::ApiError;
use axum::{
    ServiceExt,
    body::Body,
    extract::{ConnectInfo, Request},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
    routing::get,
};
use colored::Colorize;
use include_dir::{Dir, include_dir};
use sentry_tower::SentryHttpLayer;
use serde_json::json;
use sha2::Digest;
use std::{net::SocketAddr, path::Path, sync::Arc, time::Instant};
use tikv_jemallocator::Jemalloc;
use tokio::sync::RwLock;
use tower::Layer;
use tower_cookies::CookieManagerLayer;
use tower_http::{cors::CorsLayer, normalize_path::NormalizePathLayer};
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa_axum::router::OpenApiRouter;

mod cache;
mod captcha;
mod database;
mod env;
mod mail;
mod models;
mod response;
mod routes;
mod s3;
mod schedules;
mod telemetry;
mod utils;

#[global_allocator]
static ALLOC: Jemalloc = Jemalloc;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const GIT_COMMIT: &str = env!("CARGO_GIT_COMMIT");
const FRONTEND_ASSETS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../frontend/.output/public");

pub type GetIp = axum::extract::Extension<std::net::IpAddr>;

async fn handle_request(
    connect_info: ConnectInfo<SocketAddr>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let ip = crate::utils::extract_ip(req.headers()).unwrap_or_else(|| connect_info.ip());

    req.extensions_mut().insert(ip);

    tracing::info!(
        "http {} {}{}",
        req.method().to_string().to_lowercase(),
        req.uri().path().cyan(),
        if let Some(query) = req.uri().query() {
            format!("?{query}")
        } else {
            "".to_string()
        }
        .bright_cyan()
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

#[tokio::main]
async fn main() {
    let (_env_guard, env) = env::Env::parse();

    let _guard = sentry::init((
        env.sentry_url.clone(),
        sentry::ClientOptions {
            server_name: env.server_name.clone().map(|s| s.into()),
            release: Some(format!("{VERSION}:{GIT_COMMIT}").into()),
            traces_sample_rate: 1.0,
            ..Default::default()
        },
    ));

    let s3 = Arc::new(s3::S3::new(&env).await);
    let database = Arc::new(database::Database::new(&env).await);
    let cache = Arc::new(cache::Cache::new(&env).await);
    let env = Arc::new(env);

    let state = Arc::new(routes::AppState {
        start_time: Instant::now(),
        version: format!("{VERSION}:{GIT_COMMIT}"),

        github_releases: RwLock::new(Vec::new()),
        client: reqwest::Client::builder()
            .user_agent(format!("blueprint api/{}", VERSION))
            .build()
            .unwrap(),

        database: database.clone(),
        cache: cache.clone(),
        s3,
        telemetry: telemetry::TelemetryLogger::new(database, cache, env.clone()),
        env: env.clone(),
        mail: Arc::new(mail::Mail::new(env.clone())),
        captcha: Arc::new(captcha::Captcha::new(env)),
    });

    {
        let state = state.clone();

        tokio::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;

                if let Err(err) = state.telemetry.process().await {
                    tracing::error!("failed to process telemetry: {:#?}", err);
                    sentry_anyhow::capture_anyhow(&err);
                }
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
            "/api",
            get(|| async move {
                let mut headers = HeaderMap::new();

                headers.insert("Content-Type", "text/html".parse().unwrap());

                (StatusCode::OK, headers, include_str!("../static/api.html"))
            }),
        )
        .route(
            // for compatibility reasons
            "/send/{panel}/{data}",
            get(|| async move { (StatusCode::OK, axum::Json(json!({}))) }),
        )
        .fallback(|req: Request<Body>| async move {
            if !req.uri().path().starts_with("/api") {
                let path = Path::new(&req.uri().path()[1..]);

                let entry = match FRONTEND_ASSETS.get_entry(path) {
                    Some(entry) => entry,
                    None => FRONTEND_ASSETS.get_entry("404.html").unwrap(),
                };

                let file = match entry {
                    include_dir::DirEntry::File(file) => file,
                    include_dir::DirEntry::Dir(dir) => match dir.get_file("index.html") {
                        Some(index_file) => index_file,
                        None => FRONTEND_ASSETS.get_file("404.html").unwrap(),
                    },
                };

                return Response::builder()
                    .header(
                        "Content-Type",
                        match infer::get(file.contents()) {
                            Some(kind) => kind.mime_type(),
                            _ => match file.path().extension() {
                                Some(ext) => match ext.to_str() {
                                    Some("html") => "text/html",
                                    Some("js") => "application/javascript",
                                    Some("css") => "text/css",
                                    Some("json") => "application/json",
                                    _ => "application/octet-stream",
                                },
                                None => "application/octet-stream",
                            },
                        },
                    )
                    .body(Body::from(file.contents()))
                    .unwrap();
            }

            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header("Content-Type", "application/json")
                .body(Body::from(
                    ApiError::new(&["route not found"]).to_value().to_string(),
                ))
                .unwrap()
        })
        .layer(CorsLayer::very_permissive())
        .layer(axum::middleware::from_fn(handle_request))
        .layer(CookieManagerLayer::new())
        .route_layer(axum::middleware::from_fn(handle_etag))
        .route_layer(SentryHttpLayer::new().enable_transaction())
        .with_state(state.clone());

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", &state.env.bind, state.env.port))
        .await
        .unwrap();

    tracing::info!(
        "{} listening on {} {}",
        "http server".bright_red(),
        state.env.bind.cyan(),
        format!(
            "(app@{}, {}ms)",
            VERSION,
            state.start_time.elapsed().as_millis()
        )
        .bright_black()
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
        ServiceExt::<Request>::into_make_service_with_connect_info::<SocketAddr>(
            NormalizePathLayer::trim_trailing_slash().layer(router),
        ),
    )
    .await
    .unwrap();
}
