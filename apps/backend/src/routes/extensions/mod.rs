use super::State;
use utoipa_axum::{router::OpenApiRouter, routes};

mod _extension_;
mod latest;

mod get {
    use crate::{
        models::extension::Extension,
        response::{ApiResponse, ApiResponseResult},
        routes::GetState,
    };

    #[utoipa::path(get, path = "/", responses(
        (status = OK, body = Vec<crate::models::extension::ApiExtension>),
    ))]
    pub async fn route(state: GetState) -> ApiResponseResult {
        let data = state
            .cache
            .cached("extensions::all", 300, || async {
                Extension::all(&state.database).await.map_err(|e| e.into())
            })
            .await?;

        ApiResponse::json(
            data.into_iter()
                .map(|extension| extension.into_api_object())
                .collect::<Vec<_>>(),
        )
        .ok()
    }
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .routes(routes!(get::route))
        .nest("/{extension}", _extension_::router(state))
        .nest("/latest", latest::router(state))
        .with_state(state.clone())
}
