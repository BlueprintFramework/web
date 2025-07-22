use super::{ApiError, GetState, State};
use crate::telemetry::TelemetryData;
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
                            return (
                                StatusCode::BAD_REQUEST,
                                axum::Json(ApiError::new(&["invalid ip"]).to_value()),
                            );
                        }
                    };

                    let telemetry = state.telemetry.log(ip, data).await;
                    if telemetry.is_none() {
                        return (
                            StatusCode::TOO_MANY_REQUESTS,
                            axum::Json(ApiError::new(&["too many requests"]).to_value()),
                        );
                    }

                    (StatusCode::OK, axum::Json(json!({})))
                },
            ),
        )
        .with_state(state.clone())
}
