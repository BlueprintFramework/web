use crate::{
    models::{user::User, user_session::UserSession},
    response::ApiResponse,
};

use super::{GetState, State};
use axum::{
    extract::Query,
    http::{HeaderMap, StatusCode},
    routing::get,
};
use rand::distr::SampleString;
use serde::Deserialize;
use serde_json::json;
use tower_cookies::{Cookie, Cookies};
use utoipa_axum::router::OpenApiRouter;

#[derive(Deserialize)]
struct Params {
    code: String,
    state: String,
}

pub const STATE_MODE_LOGIN: u16 = 0;
pub const STATE_MODE_ADD: u16 = 1;

const STATE_COOKIE: &str = "blueprint_oauth_state";
const STATE_TTL: u64 = 10 * 60;

pub async fn create_state(
    state: &crate::routes::State,
    cookies: &Cookies,
    mode: u16,
) -> Result<String, anyhow::Error> {
    let csrf_state = rand::distr::Alphanumeric.sample_string(&mut rand::rng(), 32);

    state
        .cache
        .set(&format!("oauth_state::{csrf_state}"), STATE_TTL, &mode)
        .await?;

    cookies.add(
        Cookie::build((STATE_COOKIE, csrf_state.clone()))
            .http_only(true)
            .same_site(tower_cookies::cookie::SameSite::Lax)
            .secure(state.env.app_url.starts_with("https://"))
            .path("/api/auth/github")
            .max_age(tower_cookies::cookie::time::Duration::seconds(
                STATE_TTL as i64,
            ))
            .build(),
    );

    Ok(csrf_state)
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .route(
            "/",
            get(|state: GetState, cookies: Cookies| async move {
                let csrf_state = create_state(&state, &cookies, STATE_MODE_LOGIN).await?;

                ApiResponse::new("")
                    .with_status(StatusCode::FOUND)
                    .with_header("Location", &format!("https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}/api/auth/github/callback&scope=read:user&state={csrf_state}", state.env.github_client_id, state.env.app_url))
                    .ok()
            }),
        )
        .route(
            "/callback",
            get(|state: GetState, ip: crate::GetIp, headers: HeaderMap, cookies: Cookies, params: Query<Params>| async move {
                let state_cookie = cookies.get(STATE_COOKIE).map(|cookie| cookie.value().to_owned());
                cookies.remove(Cookie::build(STATE_COOKIE).path("/api/auth/github").build());

                let cache_key = format!("oauth_state::{}", params.state);
                let mode = state.cache.get::<u16>(&cache_key).await?;

                let mode = match mode {
                    Some(mode) if state_cookie.as_deref() == Some(params.state.as_str()) => mode,
                    _ => {
                        return ApiResponse::error("oauth csrf state not found, please try again")
                            .with_status(StatusCode::UNAUTHORIZED)
                            .ok();
                    }
                };

                state.cache.invalidate(&cache_key).await?;

                let user = state.client
                    .post("https://github.com/login/oauth/access_token")
                    .header("Accept", "application/json")
                    .json(&json!({
                        "client_id": state.env.github_client_id,
                        "client_secret": state.env.github_client_secret,
                        "code": params.code,
                        "redirect_uri": format!("{}/api/auth/github/callback", state.env.app_url),
                    }))
                    .send()
                    .await?
                    .json::<OAuthResponse>()
                    .await;

                let user = match user {
                    Ok(user) => user,
                    Err(_) => {
                        return ApiResponse::new("Invalid access token returned")
                            .with_status(StatusCode::BAD_REQUEST)
                            .ok();
                    }
                };

                #[derive(Deserialize)]
                struct OAuthResponse {
                    access_token: String,
                }

                let response = state.client
                    .get("https://api.github.com/user")
                    .header("Accept", "application/vnd.github+json")
                    .header("Authorization", format!("Bearer {}", user.access_token))
                    .send()
                    .await?;

                if !response.status().is_success() {
                    let status = response.status();
                    let body = response.text().await.unwrap_or_default();

                    tracing::warn!(
                        "failed to fetch github user: {} {}",
                        status,
                        crate::utils::slice_up_to(body.trim(), 512)
                    );

                    return ApiResponse::error("unable to retrieve github user, please try again")
                        .with_status(StatusCode::BAD_REQUEST)
                        .ok();
                }

                let data = match response.json::<UserResponse>().await {
                    Ok(data) => data,
                    Err(_) => {
                        return ApiResponse::error("unable to retrieve github user, please try again")
                            .with_status(StatusCode::BAD_REQUEST)
                            .ok();
                    }
                };

                #[derive(Deserialize)]
                struct UserResponse {
                    id: i64,
                }

                if mode == STATE_MODE_ADD {
                    if let Some(session_id) = cookies.get("blueprint_session") {
                        if session_id.value().len() != 81 {
                            return ApiResponse::error("invalid authorization cookie")
                                .with_status(StatusCode::UNAUTHORIZED)
                                .ok();
                        }

                        let user = User::by_session(&state.database, session_id.value()).await;
                        let (user, session) = match user {
                            Ok(Some(data)) => data,
                            Ok(None) => {
                                return ApiResponse::error("invalid session")
                                    .with_status(StatusCode::UNAUTHORIZED)
                                    .ok();
                            }
                            Err(err) => return ApiResponse::from(err).ok(),
                        };

                        state
                            .database
                            .batch_action("update_user_session", session.id, {
                                let state = state.clone();
                                let user_agent = crate::utils::slice_up_to(
                                    headers
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
                            Cookie::build(("blueprint_session", session_id.value().to_string()))
                                .http_only(true)
                                .same_site(tower_cookies::cookie::SameSite::Lax)
                                .secure(state.env.app_url.starts_with("https://"))
                                .path("/")
                                .expires(
                                    tower_cookies::cookie::time::OffsetDateTime::now_utc()
                                        + tower_cookies::cookie::time::Duration::days(30),
                                )
                                .build(),
                        );

                        sqlx::query!(
                            "UPDATE users
                            SET github_id = $2
                            WHERE users.id = $1",
                            user.id,
                            data.id
                        )
                        .execute(state.database.write())
                        .await?;

                        ApiResponse::new("")
                            .with_status(StatusCode::FOUND)
                            .with_header("Location", &format!("{}/app/account?reason=oauth-complete", state.env.app_url))
                            .ok()
                    } else {
                        ApiResponse::error("invalid authorization method")
                            .with_status(StatusCode::UNAUTHORIZED)
                            .ok()
                    }
                } else {
                    let user = match User::by_github_id(&state.database, data.id as i64).await? {
                        Some(user) => user,
                        None => {
                            return ApiResponse::new("")
                                .with_status(StatusCode::FOUND)
                                .with_header("Location", &format!("{}/auth/register?reason=oauth", state.env.app_url))
                                .ok();
                        }
                    };

                    let key = UserSession::create(
                        &state.database,
                        user.id,
                        ip.0.into(),
                        headers
                            .get("User-Agent")
                            .map(|ua| crate::utils::slice_up_to(ua.to_str().unwrap_or("unknown"), 255))
                            .unwrap_or("unknown"),
                    )
                    .await?;

                    cookies.add(
                        Cookie::build(("blueprint_session", key))
                            .http_only(true)
                            .same_site(tower_cookies::cookie::SameSite::Strict)
                            .secure(state.env.app_url.starts_with("https://"))
                            .path("/")
                            .expires(
                                tower_cookies::cookie::time::OffsetDateTime::now_utc()
                                    + tower_cookies::cookie::time::Duration::days(30),
                            )
                            .build(),
                    );

                    ApiResponse::new("")
                        .with_status(StatusCode::FOUND)
                        .with_header("Location", &format!("{}/app", state.env.app_url))
                        .ok()
                }
            }),
        )
        .with_state(state.clone())
}
