use super::State;
use utoipa_axum::{router::OpenApiRouter, routes};

mod get {
    use crate::{
        response::{ApiResponse, ApiResponseResult},
        routes::GetState,
    };
    use serde::{Deserialize, Serialize};
    use std::collections::BTreeMap;
    use utoipa::ToSchema;

    #[derive(ToSchema, Serialize, Deserialize)]
    struct ResponseFlag {
        pub enabled: f64,
        pub disabled: f64,
    }

    #[utoipa::path(get, path = "/", responses(
        (status = OK, body = inline(BTreeMap<String, ResponseFlag>)),
    ))]
    pub async fn route(state: GetState) -> ApiResponseResult {
        let flags = state
            .cache
            .cached("stats::flags", 3600, || async {
                let mut flags: BTreeMap<String, ResponseFlag> = BTreeMap::new();

                let data = sqlx::query!(
                    r#"
                    SELECT
                        flag.key AS flag,
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
                    LEFT JOIN LATERAL jsonb_each(data->'blueprint'->'flags') AS flag ON true
                    WHERE
                        id IN (
                            SELECT latest_telemetry_data_id
                            FROM telemetry_panels_with_latest
                        )
                        AND created > NOW() - INTERVAL '2 days'
                        AND flag.value = 'true'
                    GROUP BY flag.key
                    "#,
                )
                .fetch_all(state.database.read())
                .await?;

                for row in data {
                    flags.insert(
                        row.flag.unwrap(),
                        ResponseFlag {
                            enabled: (row.percentage.unwrap_or_default() * 100.0).round() / 100.0,
                            disabled: 100.0
                                - (row.percentage.unwrap_or_default() * 100.0).round() / 100.0,
                        },
                    );
                }

                Ok(flags)
            })
            .await?;

        ApiResponse::new_serialized(flags).ok()
    }
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .routes(routes!(get::route))
        .with_state(state.clone())
}
