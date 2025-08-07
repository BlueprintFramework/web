use super::{ApiError, GetState, State};
use crate::{
    models::{user::User, user_session::UserSession},
    response::ApiResponse,
};
use axum::{
    body::Body,
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use tower_cookies::{Cookie, Cookies};
use utoipa_axum::{router::OpenApiRouter, routes};

mod sessions;

#[derive(Clone)]
pub enum AuthMethod {
    Session(UserSession),
}

pub type GetUser = axum::extract::Extension<User>;
pub type GetAuthMethod = axum::extract::Extension<AuthMethod>;

pub async fn auth(
    state: GetState,
    ip: crate::GetIp,
    cookies: Cookies,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if let Some(session_id) = cookies.get("blueprint_session") {
        if session_id.value().len() != 81 {
            return Ok(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_string(&ApiError::new(&["invalid authorization cookie"]))
                        .unwrap(),
                ))
                .unwrap());
        }

        let user = User::by_session(&state.database, session_id.value()).await;
        let (user, session) = match user {
            Ok(Some(data)) => data,
            Ok(None) => {
                return Ok(ApiResponse::error("invalid session")
                    .with_status(StatusCode::UNAUTHORIZED)
                    .into_response());
            }
            Err(err) => return Ok(ApiResponse::from(err).into_response()),
        };

        state
            .database
            .batch_action("update_user_session", session.id, {
                let state = state.clone();
                let user_agent = crate::utils::slice_up_to(
                    req.headers()
                        .get("User-Agent")
                        .and_then(|ua| ua.to_str().ok())
                        .unwrap_or("unknown"),
                    255,
                )
                .to_string();

                async move {
                    if let Err(err) = sqlx::query!(
                        "UPDATE user_sessions
                        SET ip = $1, user_agent = $2, last_used = NOW()
                        WHERE id = $3",
                        sqlx::types::ipnetwork::IpNetwork::from(ip.0),
                        user_agent,
                        session.id,
                    )
                    .execute(state.database.write())
                    .await
                    {
                        tracing::warn!(user = user.id, "failed to update user session: {:#?}", err);
                        sentry::capture_error(&err);
                    }
                }
            })
            .await;

        cookies.add(
            Cookie::build(("session", session_id.value().to_string()))
                .http_only(true)
                .same_site(tower_cookies::cookie::SameSite::Lax)
                .secure(true)
                .path("/")
                .expires(
                    tower_cookies::cookie::time::OffsetDateTime::now_utc()
                        + tower_cookies::cookie::time::Duration::days(30),
                )
                .build(),
        );

        req.extensions_mut().insert(user);
        req.extensions_mut().insert(AuthMethod::Session(session));
    } else {
        return Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header("Content-Type", "application/json")
            .body(Body::from(
                serde_json::to_string(&ApiError::new(&["invalid authorization method"])).unwrap(),
            ))
            .unwrap());
    }

    Ok(next.run(req).await)
}

mod get {
    use crate::routes::{GetState, user::GetUser};
    use serde::Serialize;
    use utoipa::ToSchema;

    #[derive(ToSchema, Serialize)]
    struct ResponseExtensions {
        total: i64,
        unlisted: i64,
    }

    #[derive(ToSchema, Serialize)]
    struct Response {
        user: crate::models::user::ApiFullUser,

        #[schema(inline)]
        extensions: ResponseExtensions,
    }

    #[utoipa::path(get, path = "/", responses(
        (status = OK, body = inline(Response))
    ), security(
        ("api_key" = [])
    ))]
    pub async fn route(state: GetState, user: GetUser) -> axum::Json<serde_json::Value> {
        let data = sqlx::query!(
            r#"
            SELECT
                COUNT(*) AS total,
                SUM(unlisted::int) AS unlisted
            FROM extensions
            WHERE extensions.author_id = $1
            "#,
            user.id
        )
        .fetch_one(state.database.read())
        .await
        .unwrap();

        axum::Json(
            serde_json::to_value(Response {
                user: user.0.into_api_full_object(),
                extensions: ResponseExtensions {
                    total: data.total.unwrap_or_default(),
                    unlisted: data.unlisted.unwrap_or_default(),
                },
            })
            .unwrap(),
        )
    }
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .routes(routes!(get::route))
        .nest("/sessions", sessions::router(state))
        .route_layer(axum::middleware::from_fn_with_state(state.clone(), auth))
        .with_state(state.clone())
}
