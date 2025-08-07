use super::State;
use utoipa_axum::{router::OpenApiRouter, routes};

mod get {
    use crate::{
        models::extension::{Extension, ExtensionStatus},
        response::{ApiResponse, ApiResponseResult},
        routes::{ApiError, GetState},
    };
    use axum::{extract::Path, http::StatusCode};
    use indexmap::IndexMap;
    use sqlx::Row;

    #[utoipa::path(get, path = "/", responses(
        (status = OK, body = IndexMap<String, f64>),
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

        let mut versions: IndexMap<String, f64> = IndexMap::new();

        let data = sqlx::query(
            r#"
            SELECT *
            FROM (
                SELECT
                    (jsonb_array_elements(mv_extension_stats.versions)->>'version') AS version,
                    (jsonb_array_elements(mv_extension_stats.versions)->>'percentage')::float8 AS percentage
                FROM mv_extension_stats
                WHERE mv_extension_stats.id = $1
            ) x
            WHERE x.version = ANY($2)
            ORDER BY x.percentage DESC
            "#,
        )
        .bind(extension.id)
        .bind(extension.versions().into_iter().collect::<Vec<_>>())
        .fetch_all(state.database.read())
        .await?;

        for row in data {
            versions.insert(
                row.get("version"),
                (row.get::<f64, &str>("percentage") * 100.0).round() / 100.0,
            );
        }

        ApiResponse::json(versions).ok()
    }
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .routes(routes!(get::route))
        .with_state(state.clone())
}
