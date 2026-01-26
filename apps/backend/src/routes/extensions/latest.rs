use super::State;
use utoipa_axum::{router::OpenApiRouter, routes};

mod get {
    use crate::{
        models::extension::Extension,
        response::{ApiResponse, ApiResponseResult},
        routes::GetState,
    };
    use indexmap::IndexMap;

    #[utoipa::path(get, path = "/", responses(
        (status = OK, body = IndexMap<String, String>)
    ))]
    pub async fn route(state: GetState) -> ApiResponseResult {
        let data = state
            .cache
            .cached("extensions::all", 300, || async {
                Extension::all(&state.database).await.map_err(|e| e.into())
            })
            .await?;

        let mut latest_versions: IndexMap<String, String> = IndexMap::new();
        for extension in data {
            let versions = extension.versions();

            if let Some(version) = versions.first() {
                latest_versions.insert(extension.identifier.clone(), version.to_string());
            }
        }

        ApiResponse::new_serialized(latest_versions).ok()
    }
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .routes(routes!(get::route))
        .with_state(state.clone())
}
