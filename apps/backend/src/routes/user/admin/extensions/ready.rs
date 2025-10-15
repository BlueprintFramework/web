use super::State;
use utoipa_axum::{router::OpenApiRouter, routes};

mod get {
    use crate::{
        models::{Pagination, PaginationParams, extension::Extension},
        response::{ApiResponse, ApiResponseResult},
        routes::{ApiError, GetState},
    };
    use axum::{extract::Query, http::StatusCode};
    use serde::Serialize;
    use utoipa::ToSchema;

    #[derive(ToSchema, Serialize)]
    struct Response {
        #[schema(inline)]
        extensions: Pagination<crate::models::extension::ApiFullExtension>,
    }

    #[utoipa::path(get, path = "/", responses(
        (status = OK, body = inline(Response)),
    ), params(
        (
            "page" = i64, Query,
            description = "The page number",
            example = "1",
        ),
        (
            "per_page" = i64, Query,
            description = "The number of items per page",
            example = "10",
        ),
    ))]
    pub async fn route(
        state: GetState,
        Query(params): Query<PaginationParams>,
    ) -> ApiResponseResult {
        if let Err(errors) = crate::utils::validate_data(&params) {
            return ApiResponse::json(ApiError::new_strings_value(errors))
                .with_status(StatusCode::UNAUTHORIZED)
                .ok();
        }

        let extensions = Extension::all_admin_ready_with_pagination(
            &state.database,
            params.page,
            params.per_page,
        )
        .await?;

        ApiResponse::json(Response {
            extensions: Pagination {
                total: extensions.total,
                per_page: extensions.per_page,
                page: extensions.page,
                data: extensions
                    .data
                    .into_iter()
                    .map(|extension| extension.into_api_full_object(&state.env))
                    .collect(),
            },
        })
        .ok()
    }
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .routes(routes!(get::route))
        .with_state(state.clone())
}
