use serde::Serialize;
use std::{sync::Arc, time::Instant};
use tokio::sync::RwLock;
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;

mod extensions;
mod latest;
mod stats;
mod telemetry;
pub mod user;

#[derive(ToSchema, Serialize)]
pub struct ApiError<'a> {
    pub errors: &'a [&'a str],
}

impl<'a> ApiError<'a> {
    pub fn new(errors: &'a [&'a str]) -> Self {
        Self { errors }
    }

    pub fn to_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}

pub struct AppState {
    pub start_time: Instant,
    pub version: String,

    pub github_releases: RwLock<Vec<String>>,

    pub database: Arc<crate::database::Database>,
    pub cache: Arc<crate::cache::Cache>,
    pub telemetry: crate::telemetry::TelemetryLogger,
    pub env: Arc<crate::env::Env>,
}

impl AppState {
    pub fn client(&self) -> reqwest::Client {
        reqwest::Client::builder()
            .user_agent(format!("blueprint api/{}", self.version))
            .build()
            .unwrap()
    }
}

pub type State = Arc<AppState>;
pub type GetState = axum::extract::State<State>;

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .nest("/telemetry", telemetry::router(state))
        .nest("/latest", latest::router(state))
        .nest("/extensions", extensions::router(state))
        .nest("/stats", stats::router(state))
        .nest("/user", user::router(state))
        .with_state(state.clone())
}
