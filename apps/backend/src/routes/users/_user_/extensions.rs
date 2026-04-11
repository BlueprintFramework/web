use super::State;
use utoipa_axum::{router::OpenApiRouter, routes};

mod get {
    use crate::{
        models::extension::Extension,
        response::{ApiResponse, ApiResponseResult},
        routes::{GetState, users::_user_::GetParamUser},
    };

    #[utoipa::path(get, path = "/", responses(
        (status = OK, body = Vec<crate::models::extension::ApiExtension>),
    ), params(
        (
            "user" = i32,
            description = "the id of the user"
        ),
    ))]
    pub async fn route(state: GetState, user: GetParamUser) -> ApiResponseResult {
        let data = state
            .cache
            .cached("extensions::all", 300, || async {
                Extension::all(&state.database).await.map_err(|e| e.into())
            })
            .await?;

        ApiResponse::new_serialized(
            data.into_iter()
                .filter(|extension| extension.author.id == user.id)
                .map(|extension| extension.into_api_object(&state.env))
                .collect::<Vec<_>>(),
        )
        .ok()
    }
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .routes(routes!(get::route))
        .with_state(state.clone())
}
