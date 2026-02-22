use super::{GetState, State};
use crate::{response::ApiResponse, telemetry::TelemetryData};
use axum::{
    http::{HeaderMap, StatusCode},
    routing::post,
};
use serde_json::json;
use utoipa_axum::router::OpenApiRouter;

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .route(
            "/",
            post(
                |state: GetState, headers: HeaderMap, axum::Json::<TelemetryData>(data)| async move {
                    let ip = match crate::utils::extract_ip(&headers) {
                        Some(ip) => ip,
                        None => {
                            return ApiResponse::error("invalid ip")
                                .with_status(StatusCode::BAD_REQUEST)
                                .ok();
                        }
                    };

                    state.cache.ratelimit("telemetry", state.env.telemetry_ratelimit_per_day, 24 * 60 * 60, ip.to_string()).await?;

                    state.telemetry.log(ip, data).await;

                    ApiResponse::new_serialized(json!({})).ok()
                },
            ),
        )
        .with_state(state.clone())
}
