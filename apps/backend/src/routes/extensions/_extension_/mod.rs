mod versions;

use super::State;
use utoipa_axum::{router::OpenApiRouter, routes};

mod get {
    use crate::{
        models::extension::{Extension, ExtensionStatus},
        routes::{ApiError, GetState},
    };
    use axum::{extract::Path, http::StatusCode};

    #[utoipa::path(get, path = "/", responses(
        (status = OK, body = crate::models::extension::ApiExtension),
        (status = NOT_FOUND, body = inline(ApiError)),
    ), params(
        ("extension" = String, Path, description = "the extension identifier or id")
    ))]
    pub async fn route(
        state: GetState,
        Path(extension): Path<String>,
    ) -> (StatusCode, axum::Json<serde_json::Value>) {
        let extension = state
            .cache
            .cached(&format!("extensions::{}", extension), 300, || async {
                match extension.parse::<i32>() {
                    Ok(id) => {
                        if id < 1 {
                            None
                        } else {
                            Extension::by_id(&state.database, id).await
                        }
                    }
                    Err(_) => Extension::by_identifier(&state.database, &extension).await,
                }
            })
            .await;

        match extension {
            Some(extension) => {
                if !extension.unlisted && extension.status == ExtensionStatus::Approved {
                    (
                        StatusCode::OK,
                        axum::Json(serde_json::to_value(extension.into_api_object()).unwrap()),
                    )
                } else {
                    (
                        StatusCode::NOT_FOUND,
                        axum::Json(
                            serde_json::to_value(ApiError::new(&["extension not found"])).unwrap(),
                        ),
                    )
                }
            }
            None => (
                StatusCode::NOT_FOUND,
                axum::Json(serde_json::to_value(ApiError::new(&["extension not found"])).unwrap()),
            ),
        }
    }
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .nest("/versions", versions::router(state))
        .routes(routes!(get::route))
        .with_state(state.clone())
}
