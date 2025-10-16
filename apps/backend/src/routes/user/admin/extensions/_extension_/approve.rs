use super::State;
use utoipa_axum::{router::OpenApiRouter, routes};

mod post {
    use crate::{
        models::extension::ExtensionStatus,
        response::{ApiResponse, ApiResponseResult},
        routes::{ApiError, GetState, user::admin::extensions::_extension_::GetExtension},
    };
    use axum::http::StatusCode;
    use serde::Serialize;
    use utoipa::ToSchema;

    #[derive(ToSchema, Serialize)]
    struct Response {}

    #[utoipa::path(post, path = "/", responses(
        (status = OK, body = inline(Response)),
        (status = NOT_FOUND, body = ApiError),
    ), params(
        ("extension" = String, Path, description = "the extension identifier or id")
    ))]
    pub async fn route(state: GetState, extension: GetExtension) -> ApiResponseResult {
        if extension.status != ExtensionStatus::Ready {
            return ApiResponse::error("unable to mark non-ready extension as approved")
                .with_status(StatusCode::CONFLICT)
                .ok();
        }

        sqlx::query!(
            "UPDATE extensions
            SET status = 'APPROVED', identifier = $2
            WHERE extensions.id = $1",
            extension.id,
            if let Some((prefix, _)) = extension.identifier.rsplit_once(':') {
                prefix
            } else {
                &extension.identifier
            }
        )
        .execute(state.database.write())
        .await?;
        state.cache.clear_extension(&extension).await?;

        ApiResponse::json(Response {}).ok()
    }
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .routes(routes!(post::route))
        .with_state(state.clone())
}
