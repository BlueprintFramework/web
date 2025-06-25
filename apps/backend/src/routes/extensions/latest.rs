use super::State;
use utoipa_axum::{router::OpenApiRouter, routes};

mod index {
    use crate::{models::extension::Extension, routes::GetState};
    use axum::http::StatusCode;
    use indexmap::IndexMap;

    #[utoipa::path(get, path = "/", responses(
        (status = OK, body = IndexMap<String, String>)
    ))]
    pub async fn route(state: GetState) -> (StatusCode, axum::Json<serde_json::Value>) {
        let data = state
            .cache
            .cached("extensions::all", 300, || async {
                Extension::all(&state.database).await
            })
            .await;

        let mut latest_versions: IndexMap<String, String> = IndexMap::new();
        for extension in data {
            let versions = extension.versions();

            if let Some(version) = versions.first() {
                latest_versions.insert(extension.identifier.clone(), version.to_string());
            }
        }

        (
            StatusCode::OK,
            axum::Json(serde_json::to_value(&latest_versions).unwrap()),
        )
    }
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .routes(routes!(index::route))
        .with_state(state.clone())
}
