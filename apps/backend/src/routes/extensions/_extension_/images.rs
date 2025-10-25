use super::State;
use utoipa_axum::{router::OpenApiRouter, routes};

mod get {
    use crate::{
        models::{
            extension::{Extension, ExtensionStatus},
            extension_image::ExtensionImage,
        },
        response::{ApiResponse, ApiResponseResult},
        routes::{ApiError, GetState},
    };
    use axum::{extract::Path, http::StatusCode};

    #[utoipa::path(get, path = "/", responses(
        (status = OK, body = Vec<crate::models::extension_image::ApiExtensionImage>),
        (status = NOT_FOUND, body = ApiError),
    ), params(
        ("extension" = String, Path, description = "the extension identifier or id")
    ))]
    pub async fn route(state: GetState, Path(extension): Path<String>) -> ApiResponseResult {
        let extension = match state
            .cache
            .cached(&format!("extensions::{extension}"), 300, || async {
                match extension.parse::<i32>() {
                    Ok(id) => {
                        if id < 1 {
                            Ok(None)
                        } else {
                            Extension::by_id(&state.database, id)
                                .await
                                .map_err(|e| e.into())
                        }
                    }
                    Err(_) => Extension::by_identifier(&state.database, &extension)
                        .await
                        .map_err(|e| e.into()),
                }
            })
            .await?
        {
            Some(extension) => {
                if !extension.unlisted && extension.status == ExtensionStatus::Approved {
                    extension
                } else {
                    return ApiResponse::error("extension not found")
                        .with_status(StatusCode::NOT_FOUND)
                        .ok();
                }
            }
            None => {
                return ApiResponse::error("extension not found")
                    .with_status(StatusCode::NOT_FOUND)
                    .ok();
            }
        };

        let extension_images =
            ExtensionImage::all_by_extension_id(&state.database, extension.id).await?;

        ApiResponse::json(
            extension_images
                .into_iter()
                .map(|extension_image| extension_image.into_api_object(&state.env))
                .collect::<Vec<_>>(),
        )
        .ok()
    }
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .routes(routes!(get::route))
        .with_state(state.clone())
}
