use super::State;
use utoipa_axum::{router::OpenApiRouter, routes};

mod post {
    use crate::{
        models::{
            user::{ApiFullUser, User},
            user_session::UserSession,
        },
        response::{ApiResponse, ApiResponseResult},
        routes::{ApiError, GetState},
    };
    use axum::http::StatusCode;
    use serde::{Deserialize, Serialize};
    use tower_cookies::{Cookie, Cookies};
    use utoipa::ToSchema;
    use validator::Validate;

    #[derive(ToSchema, Validate, Deserialize)]
    pub struct Payload {
        email: String,
        #[validate(length(max = 512))]
        #[schema(max_length = 512)]
        password: String,

        captcha: Option<String>,
    }

    #[derive(ToSchema, Serialize)]
    struct Response {
        user: ApiFullUser,
    }

    #[utoipa::path(post, path = "/", responses(
        (status = OK, body = inline(Response)),
        (status = BAD_REQUEST, body = ApiError),
    ), request_body = inline(Payload))]
    pub async fn route(
        state: GetState,
        ip: crate::GetIp,
        headers: axum::http::HeaderMap,
        cookies: Cookies,
        axum::Json(data): axum::Json<Payload>,
    ) -> ApiResponseResult {
        if let Err(errors) = crate::utils::validate_data(&data) {
            return ApiResponse::json(ApiError::new_strings_value(errors))
                .with_status(StatusCode::BAD_REQUEST)
                .ok();
        }

        if let Err(error) = state.captcha.verify(ip, data.captcha).await {
            return ApiResponse::error(&error)
                .with_status(StatusCode::BAD_REQUEST)
                .ok();
        }

        let user =
            match User::by_email_password(&state.database, &data.email, &data.password).await? {
                Some(user) => user,
                None => {
                    return ApiResponse::error("invalid name or password")
                        .with_status(StatusCode::BAD_REQUEST)
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

        ApiResponse::json(Response {
            user: user.into_api_full_object(),
        })
        .ok()
    }
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .routes(routes!(post::route))
        .with_state(state.clone())
}
