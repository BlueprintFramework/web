use super::State;
use crate::{response::ApiResponse, routes::user::GetUser};
use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use utoipa_axum::router::OpenApiRouter;

mod extensions;

pub async fn auth(user: GetUser, mut req: Request, next: Next) -> Result<Response, StatusCode> {
    if !user.admin {
        return Ok(ApiResponse::error("unauthorized")
            .with_status(StatusCode::UNAUTHORIZED)
            .into_response());
    }

    req.extensions_mut().insert(user.0);

    Ok(next.run(req).await)
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .nest("/extensions", extensions::router(state))
        .route_layer(axum::middleware::from_fn_with_state(state.clone(), auth))
        .with_state(state.clone())
}
