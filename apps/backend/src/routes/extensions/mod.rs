mod _extension_;
mod latest;

use super::State;
use utoipa_axum::{router::OpenApiRouter, routes};

mod index {
    use crate::{models::extension::Extension, routes::GetState};

    #[utoipa::path(get, path = "/", responses(
        (status = OK, body = Vec<Extension>)
    ))]
    pub async fn route(state: GetState) -> axum::Json<serde_json::Value> {
        let data = state
            .cache
            .cached("extensions::all", 300, || async {
                Extension::all(&state.database).await
            })
            .await;

        axum::Json(serde_json::to_value(data).unwrap())
    }
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .nest("/{extension}", _extension_::router(state))
        .nest("/latest", latest::router(state))
        .routes(routes!(index::route))
        .with_state(state.clone())
}
