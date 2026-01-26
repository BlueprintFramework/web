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

                    let telemetry = state.telemetry.log(ip, data).await;
                    if telemetry.is_none() {
                        return ApiResponse::error("too many requests")
                            .with_status(StatusCode::TOO_MANY_REQUESTS)
                            .ok();
                    }

                    ApiResponse::new_serialized(json!({})).ok()
                },
            ),
        )
        .with_state(state.clone())
}
