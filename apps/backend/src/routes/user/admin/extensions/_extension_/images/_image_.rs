use super::State;
use utoipa_axum::{router::OpenApiRouter, routes};

mod delete {
    use crate::{
        models::extension_image::ExtensionImage,
        response::{ApiResponse, ApiResponseResult},
        routes::{ApiError, GetState, user::admin::extensions::_extension_::GetExtension},
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
        ("extension" = String, Path, description = "the extension identifier or id"),
        ("image" = i32, Path, description = "the image id")
    ))]
    pub async fn route(
        state: GetState,
        extension: GetExtension,
        Path((_extension, image)): Path<(String, i32)>,
    ) -> ApiResponseResult {
        let extension_image =
            match ExtensionImage::by_extension_id_id(&state.database, extension.id, image).await? {
                Some(extension_image) => extension_image,
                None => {
                    return ApiResponse::error("extension image not found")
                        .with_status(StatusCode::NOT_FOUND)
                        .ok();
                }
            };

        extension_image.delete(&state.database, &state.s3).await?;

        ApiResponse::new_serialized(Response {}).ok()
    }
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .routes(routes!(delete::route))
        .with_state(state.clone())
}
