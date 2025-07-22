use super::State;
use utoipa_axum::{router::OpenApiRouter, routes};

mod get {
    use crate::routes::GetState;
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;

    #[derive(ToSchema, Serialize, Deserialize)]
    struct ResponseExtensions {
        total: i64,
        max: i32,
        average: f64,
    }

    #[derive(ToSchema, Serialize, Deserialize)]
    struct Response {
        total: i64,
        docker: i64,
        standalone: i64,

        #[schema(inline)]
        extensions: ResponseExtensions,
    }

    #[utoipa::path(get, path = "/", responses(
        (status = OK, body = inline(Response)),
    ))]
    pub async fn route(state: GetState) -> axum::Json<serde_json::Value> {
        let response = state
            .cache
            .cached("stats::panels", 3600, || async {
                let data = sqlx::query!(
                    r#"
                    SELECT
                        COUNT(*) AS total_panels,
                        SUM((data->'blueprint'->>'docker')::boolean::int) AS docker_panels,
                        SUM(jsonb_array_length(data->'blueprint'->'extensions')) AS sum_extensions,
                        MAX(jsonb_array_length(data->'blueprint'->'extensions')) AS max_extensions,
                        AVG(jsonb_array_length(data->'blueprint'->'extensions'))::float8 AS avg_extensions
                    FROM telemetry_data
                    WHERE
                        id IN (
                            SELECT latest_telemetry_data_id
                            FROM telemetry_panels_with_latest
                        )
                        AND created > NOW() - INTERVAL '2 days'
                    "#,
                )
                .fetch_one(state.database.read())
                .await
                .unwrap();

                Response {
                    total: data.total_panels.unwrap_or_default(),
                    docker: data.docker_panels.unwrap_or_default(),
                    standalone: data.total_panels.unwrap_or_default() - data.docker_panels.unwrap_or_default(),

                    extensions: ResponseExtensions {
                        total: data.sum_extensions.unwrap_or_default(),
                        max: data.max_extensions.unwrap_or_default(),
                        average: (data.avg_extensions.unwrap_or_default() * 100.0).round() / 100.0,
                    },
                }
            })
            .await;

        axum::Json(serde_json::to_value(response).unwrap())
    }
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .routes(routes!(get::route))
        .with_state(state.clone())
}
