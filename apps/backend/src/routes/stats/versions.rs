use super::State;
use utoipa_axum::{router::OpenApiRouter, routes};

mod get {
    use crate::routes::GetState;
    use indexmap::IndexMap;

    #[utoipa::path(get, path = "/", responses(
        (status = OK, body = inline(HashMap<String, f64>)),
    ))]
    pub async fn route(state: GetState) -> axum::Json<serde_json::Value> {
        let versions = state
            .cache
            .cached("stats::versions", 3600, || async {
                let mut versions: IndexMap<String, f64> = IndexMap::new();

                let data = sqlx::query!(
                    r#"
                    SELECT version, percentage
                    FROM (
                        SELECT
                            data->'blueprint'->>'version' AS version,
                            (COUNT(*) * 100.0 / (
                                SELECT COUNT(*)
                                FROM telemetry_data
                                WHERE id IN (
                                    SELECT latest_telemetry_data_id
                                    FROM telemetry_panels_with_latest
                                )
                                AND created > NOW() - INTERVAL '2 days'
                            ))::float8 AS percentage
                        FROM telemetry_data
                        WHERE
                            id IN (
                                SELECT latest_telemetry_data_id
                                FROM telemetry_panels_with_latest
                            )
                            AND created > NOW() - INTERVAL '2 days'
                        GROUP BY version
                    ) x
                    WHERE x.version LIKE 'beta-202_-__'
                    ORDER BY x.percentage DESC
                    "#,
                )
                .fetch_all(state.database.read())
                .await
                .unwrap();

                let total = data.iter().map(|row| row.percentage.unwrap()).sum::<f64>();
                for row in data {
                    versions.insert(
                        row.version.unwrap(),
                        (row.percentage.unwrap() / total * 10000.0).round() / 100.0,
                    );
                }

                versions
            })
            .await;

        axum::Json(serde_json::to_value(versions).unwrap())
    }
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .routes(routes!(get::route))
        .with_state(state.clone())
}
