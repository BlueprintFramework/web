use super::State;
use utoipa_axum::{router::OpenApiRouter, routes};

mod get {
    use crate::{
        response::{ApiResponse, ApiResponseResult},
        routes::{ApiError, GetState, user::GetUser},
    };
    use axum::http::StatusCode;
    use serde::Serialize;
    use tower_cookies::Cookies;
    use utoipa::ToSchema;

    #[derive(ToSchema, Serialize)]
    struct Response {
        redirect_url: String,
    }

    #[utoipa::path(get, path = "/", responses(
        (status = OK, body = inline(Response)),
        (status = CONFLICT, body = ApiError),
    ))]
    pub async fn route(state: GetState, cookies: Cookies, user: GetUser) -> ApiResponseResult {
        if user.github_id.is_some() {
            return ApiResponse::error("github account is already linked")
                .with_status(StatusCode::CONFLICT)
                .ok();
        }

        let csrf_state = crate::routes::auth::github::create_state(
            &state,
            &cookies,
            crate::routes::auth::github::STATE_MODE_ADD,
        )
        .await?;

        ApiResponse::new_serialized(Response {
            redirect_url: format!("https://github.com/login/oauth/authorize?client_id={}&state={csrf_state}&redirect_uri={}/api/auth/github/callback&scope=read:user", state.env.github_client_id, state.env.app_url)
        })
        .ok()
    }
}

mod delete {
    use crate::{
        response::{ApiResponse, ApiResponseResult},
        routes::{ApiError, GetState, user::GetUser},
    };
    use axum::http::StatusCode;
    use serde::Serialize;
    use utoipa::ToSchema;

    #[derive(ToSchema, Serialize)]
    struct Response {}

    #[utoipa::path(delete, path = "/", responses(
        (status = OK, body = inline(Response)),
        (status = BAD_REQUEST, body = ApiError),
        (status = CONFLICT, body = ApiError),
        (status = UNAUTHORIZED, body = ApiError),
    ))]
    pub async fn route(state: GetState, user: GetUser) -> ApiResponseResult {
        if user.github_id.is_none() {
            return ApiResponse::error("github account is not linked")
                .with_status(StatusCode::CONFLICT)
                .ok();
        }

        sqlx::query!(
            "UPDATE users
            SET github_id = NULL
            WHERE users.id = $1",
            user.id
        )
        .execute(state.database.write())
        .await?;

        ApiResponse::new_serialized(Response {}).ok()
    }
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .routes(routes!(get::route))
        .routes(routes!(delete::route))
        .with_state(state.clone())
}
