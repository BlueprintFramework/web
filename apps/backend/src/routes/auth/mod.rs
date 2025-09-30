use super::State;
use crate::{response::ApiResponse, routes::GetState};
use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use utoipa_axum::router::OpenApiRouter;

mod login;
mod password;
mod register;

pub async fn auth(state: GetState, req: Request, next: Next) -> Result<Response, StatusCode> {
    if !state.env.app_auth {
        return Ok(ApiResponse::error("route not found")
            .with_status(StatusCode::NOT_FOUND)
            .into_response());
    }

    Ok(next.run(req).await)
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .nest("/login", login::router(state))
        .nest("/register", register::router(state))
        .nest("/password", password::router(state))
        .route_layer(axum::middleware::from_fn_with_state(state.clone(), auth))
        .with_state(state.clone())
}
