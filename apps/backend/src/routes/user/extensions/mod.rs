use super::State;
use utoipa_axum::{router::OpenApiRouter, routes};

mod _extension_;

mod get {
    use crate::{
        models::{Pagination, PaginationParams, extension::Extension},
        response::{ApiResponse, ApiResponseResult},
        routes::{ApiError, GetState, user::GetUser},
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
        user: GetUser,
        Query(params): Query<PaginationParams>,
    ) -> ApiResponseResult {
        if let Err(errors) = crate::utils::validate_data(&params) {
            return ApiResponse::json(ApiError::new_strings_value(errors))
                .with_status(StatusCode::UNAUTHORIZED)
                .ok();
        }

        let extensions = Extension::by_author_id_with_pagination(
            &state.database,
            user.id,
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

mod post {
    use crate::{
        models::extension::{
            Extension, ExtensionPlatform, ExtensionPlatformData, ExtensionType,
            MinimalExtensionPlatformData,
        },
        response::{ApiResponse, ApiResponseResult},
        routes::{ApiError, GetState, user::GetUser},
    };
    use axum::http::StatusCode;
    use serde::{Deserialize, Serialize};
    use std::collections::BTreeMap;
    use utoipa::ToSchema;
    use validator::Validate;

    #[derive(ToSchema, Validate, Deserialize)]
    pub struct Payload {
        #[validate(length(min = 3, max = 63))]
        #[schema(min_length = 3, max_length = 63)]
        name: String,
        #[validate(
            length(min = 3, max = 48),
            regex(path = "*crate::models::extension::IDENTIFIER_REGEX")
        )]
        #[schema(min_length = 3, max_length = 48)]
        #[schema(pattern = "^[a-z]+$")]
        identifier: String,
        r#type: ExtensionType,
        unlisted: bool,
        #[validate(length(min = 3, max = 255))]
        #[schema(min_length = 3, max_length = 255)]
        summary: String,
        #[validate(length(max = 1024))]
        #[schema(max_length = 1024)]
        description: Option<String>,
        platforms: BTreeMap<ExtensionPlatform, MinimalExtensionPlatformData>,
    }

    #[derive(ToSchema, Serialize)]
    struct Response {}

    #[utoipa::path(post, path = "/", responses(
        (status = OK, body = inline(Response)),
    ), request_body = inline(Payload))]
    pub async fn route(
        state: GetState,
        user: GetUser,
        axum::Json(mut data): axum::Json<Payload>,
    ) -> ApiResponseResult {
        if let Err(errors) = crate::utils::validate_data(&data) {
            return ApiResponse::json(ApiError::new_strings_value(errors))
                .with_status(StatusCode::BAD_REQUEST)
                .ok();
        }

        if user.suspended {
            return ApiResponse::error("you are not allowed to create new extensions")
                .with_status(StatusCode::CONFLICT)
                .ok();
        }

        let mut platforms = BTreeMap::new();

        if let Some(builtbybit) = data.platforms.remove(&ExtensionPlatform::Builtbybit) {
            let url = match reqwest::Url::parse(&builtbybit.url) {
                Ok(url) => url,
                Err(_) => return ApiResponse::error("invalid builtbybit url").ok(),
            };

            if url.host_str() != Some("builtbybit.com") || !url.path().starts_with("/resources/") {
                return ApiResponse::error("invalid builtbybit url").ok();
            }

            let mut segments = match url.path_segments() {
                Some(segments) => segments,
                None => return ApiResponse::error("invalid builtbybit url").ok(),
            };

            if let Some(resource) = segments.next_back() {
                let product_id = resource
                    .split('.')
                    .next_back()
                    .unwrap_or(resource)
                    .trim_end_matches(|c: char| !c.is_ascii_digit())
                    .parse::<u32>();

                if product_id.is_err() {
                    return ApiResponse::error("invalid builtbybit url").ok();
                }
            }

            platforms.insert(
                ExtensionPlatform::Builtbybit,
                ExtensionPlatformData {
                    url: builtbybit.url,
                    price: builtbybit.price,
                    currency: builtbybit.currency,
                    reviews: None,
                    rating: None,
                },
            );
        }

        if let Some(sourcexchange) = data.platforms.remove(&ExtensionPlatform::Sourcexchange) {
            let url = match reqwest::Url::parse(&sourcexchange.url) {
                Ok(url) => url,
                Err(_) => return ApiResponse::error("invalid sourcexchange url").ok(),
            };

            if url.host_str() != Some("www.sourcexchange.net")
                || !url.path().starts_with("/products/")
            {
                return ApiResponse::error("invalid sourcexchange url").ok();
            }

            platforms.insert(
                ExtensionPlatform::Sourcexchange,
                ExtensionPlatformData {
                    url: sourcexchange.url,
                    price: sourcexchange.price,
                    currency: sourcexchange.currency,
                    reviews: None,
                    rating: None,
                },
            );
        }

        if let Some(github) = data.platforms.remove(&ExtensionPlatform::Github) {
            let url = match reqwest::Url::parse(&github.url) {
                Ok(url) => url,
                Err(_) => return ApiResponse::error("invalid github url").ok(),
            };

            if url.host_str() != Some("github.com") {
                return ApiResponse::error("invalid github url").ok();
            }

            let segments = match url.path_segments() {
                Some(segments) => segments,
                None => return ApiResponse::error("invalid github url").ok(),
            };

            if segments.count() != 2 {
                return ApiResponse::error("invalid github url").ok();
            }

            platforms.insert(
                ExtensionPlatform::Github,
                ExtensionPlatformData {
                    url: github.url,
                    price: github.price,
                    currency: github.currency,
                    reviews: None,
                    rating: None,
                },
            );
        }

        match Extension::create(
            &state.database,
            user.id,
            &data.name,
            &data.identifier,
            data.r#type,
            data.unlisted,
            &data.summary,
            data.description.as_deref(),
            &platforms,
        )
        .await
        {
            Ok(_) => {}
            Err(err) if err.to_string().contains("unique constraint") => {
                return ApiResponse::error("extension with identifier already exists")
                    .with_status(StatusCode::CONFLICT)
                    .ok();
            }
            Err(err) => {
                tracing::error!("failed to create extension: {:#?}", err);

                return ApiResponse::error("failed to create extension")
                    .with_status(StatusCode::INTERNAL_SERVER_ERROR)
                    .ok();
            }
        }

        ApiResponse::json(Response {}).ok()
    }
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .routes(routes!(get::route))
        .routes(routes!(post::route))
        .nest("/{extension}", _extension_::router(state))
        .with_state(state.clone())
}
