use super::State;
use utoipa_axum::{router::OpenApiRouter, routes};

mod delete {
    use crate::{
        models::user_session::UserSession,
        response::{ApiResponse, ApiResponseResult},
        routes::{ApiError, GetState, user::GetUser},
    };
    use axum::{extract::Path, http::StatusCode};
    use serde::Serialize;
    use utoipa::ToSchema;

    #[derive(ToSchema, Serialize)]
    struct Response {}

    #[utoipa::path(delete, path = "/", responses(
        (status = OK, body = inline(Response)),
        (status = NOT_FOUND, body = ApiError),
    ), params(
        (
            "session" = i32,
            description = "The session ID",
            example = "1",
        ),
    ))]
    pub async fn route(
        state: GetState,
        user: GetUser,
        Path(session): Path<i32>,
    ) -> ApiResponseResult {
        let session = match UserSession::by_user_id_id(&state.database, user.id, session).await? {
            Some(session) => session,
            None => {
                return ApiResponse::error("session not found")
                    .with_status(StatusCode::NOT_FOUND)
                    .ok();
            }
        };

        UserSession::delete_by_id(&state.database, session.id).await?;

        ApiResponse::json(Response {}).ok()
    }
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .routes(routes!(delete::route))
        .with_state(state.clone())
}
