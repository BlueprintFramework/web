use super::State;
use utoipa_axum::{router::OpenApiRouter, routes};

mod images;
mod versions;

mod get {
    use crate::{
        models::extension::{Extension, ExtensionStatus},
        response::{ApiResponse, ApiResponseResult},
        routes::{ApiError, GetState},
    };
    use axum::{extract::Path, http::StatusCode};

    #[utoipa::path(get, path = "/", responses(
        (status = OK, body = crate::models::extension::ApiExtension),
        (status = NOT_FOUND, body = ApiError),
    ), params(
        ("extension" = String, Path, description = "the extension identifier or id")
    ))]
    pub async fn route(state: GetState, Path(extension): Path<String>) -> ApiResponseResult {
        let extension = state
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
            .await?;

        match extension {
            Some(extension) => {
                if !extension.unlisted && extension.status == ExtensionStatus::Approved {
                    ApiResponse::new_serialized(extension.into_api_object(&state.env)).ok()
                } else {
                    ApiResponse::error("extension not found")
                        .with_status(StatusCode::NOT_FOUND)
                        .ok()
                }
            }
            None => ApiResponse::error("extension not found")
                .with_status(StatusCode::NOT_FOUND)
                .ok(),
        }
    }
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .routes(routes!(get::route))
        .nest("/versions", versions::router(state))
        .nest("/images", images::router(state))
        .with_state(state.clone())
}
