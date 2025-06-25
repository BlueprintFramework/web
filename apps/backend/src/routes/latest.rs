use super::State;
use utoipa_axum::{router::OpenApiRouter, routes};

mod get {
    use crate::routes::{ApiError, GetState};
    use axum::http::StatusCode;
    use serde::Serialize;
    use utoipa::ToSchema;

    #[derive(ToSchema, Serialize)]
    struct Response<'a> {
        name: &'a str,
        history: &'a [String],
    }

    #[utoipa::path(get, path = "/", responses(
        (status = OK, body = inline(Response)),
        (status = NOT_FOUND, body = inline(ApiError)),
    ))]
    pub async fn route(state: GetState) -> (StatusCode, axum::Json<serde_json::Value>) {
        let releases = state.github_releases.read().await;

        if releases.is_empty() {
            return (
                StatusCode::NOT_FOUND,
                axum::Json(
                    serde_json::to_value(ApiError::new(&["no releases found (please retry)"]))
                        .unwrap(),
                ),
            );
        }

        (
            StatusCode::OK,
            axum::Json(
                serde_json::to_value(Response {
                    name: releases.first().unwrap(),
                    history: &releases[1..],
                })
                .unwrap(),
            ),
        )
    }
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .routes(routes!(get::route))
        .with_state(state.clone())
}
