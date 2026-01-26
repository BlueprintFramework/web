use super::State;
use utoipa_axum::{router::OpenApiRouter, routes};

mod post {
    use crate::{
        models::extension::ExtensionStatus,
        response::{ApiResponse, ApiResponseResult},
        routes::{ApiError, GetState, user::admin::extensions::_extension_::GetExtension},
    };
    use axum::http::StatusCode;
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;
    use validator::Validate;

    #[derive(ToSchema, Validate, Deserialize)]
    pub struct Payload {
        #[validate(length(min = 1, max = 255))]
        #[schema(min_length = 1, max_length = 255)]
        deny_reason: String,
    }

    #[derive(ToSchema, Serialize)]
    struct Response {}

    #[utoipa::path(post, path = "/", responses(
        (status = OK, body = inline(Response)),
        (status = NOT_FOUND, body = ApiError),
    ), params(
        ("extension" = String, Path, description = "the extension identifier or id")
    ), request_body = inline(Payload))]
    pub async fn route(
        state: GetState,
        extension: GetExtension,
        crate::Payload(data): crate::Payload<Payload>,
    ) -> ApiResponseResult {
        if let Err(errors) = crate::utils::validate_data(&data) {
            return ApiResponse::new_serialized(ApiError::new_strings_value(errors))
                .with_status(StatusCode::UNAUTHORIZED)
                .ok();
        }

        if extension.status == ExtensionStatus::Pending {
            return ApiResponse::error("unable to mark pending extension as denied")
                .with_status(StatusCode::CONFLICT)
                .ok();
        }

        sqlx::query!(
            "UPDATE extensions
            SET status = 'PENDING', deny_reason = $2
            WHERE extensions.id = $1",
            extension.id,
            data.deny_reason
        )
        .execute(state.database.write())
        .await?;
        state.cache.clear_extension(&extension).await?;

        ApiResponse::new_serialized(Response {}).ok()
    }
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .routes(routes!(post::route))
        .with_state(state.clone())
}
