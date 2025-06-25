use super::State;
use utoipa_axum::{router::OpenApiRouter, routes};

mod index {
    use crate::{models::extension::Extension, routes::ApiError, routes::GetState};
    use axum::{extract::Path, http::StatusCode};
    use indexmap::IndexMap;
    use sqlx::Row;

    #[utoipa::path(get, path = "/", responses(
        (status = OK, body = IndexMap<String, f64>),
        (status = NOT_FOUND, body = inline(ApiError)),
    ), params(
        ("extension" = String, Path, description = "the extension identifier or id")
    ))]
    pub async fn route(
        state: GetState,
        Path(extension): Path<String>,
    ) -> (StatusCode, axum::Json<serde_json::Value>) {
        let extension = match state
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
            .await
        {
            Some(extension) => extension,
            None => {
                return (
                    StatusCode::NOT_FOUND,
                    axum::Json(
                        serde_json::to_value(ApiError::new(&["extension not found"])).unwrap(),
                    ),
                );
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
        .await
        .unwrap();

        for row in data {
            versions.insert(
                row.get("version"),
                (row.get::<f64, &str>("percentage") * 100.0).round() / 100.0,
            );
        }

        (
            StatusCode::OK,
            axum::Json(serde_json::to_value(&versions).unwrap()),
        )
    }
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .routes(routes!(index::route))
        .with_state(state.clone())
}
