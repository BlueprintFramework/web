use super::State;
use crate::{
    models::extension::Extension,
    response::ApiResponse,
    routes::{GetState, user::GetUser},
};
use axum::{
    extract::{Path, Request},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use utoipa_axum::{router::OpenApiRouter, routes};

mod approve;
mod banner;
mod deny;
mod images;

pub type GetExtension = crate::extract::ConsumingExtension<Extension>;

pub async fn auth(
    state: GetState,
    user: GetUser,
    Path(extension): Path<Vec<String>>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let extension = match extension.first() {
        Some(extension) => extension,
        None => {
            return Ok(ApiResponse::error("invalid extension")
                .with_status(StatusCode::UNAUTHORIZED)
                .into_response());
        }
    };
    let extension = match state
        .cache
        .cached(&format!("extensions::{extension}"), 300, || async {
            match extension.parse::<i32>() {
                Ok(id) => {
                    if id < 1 {
                        Ok(None)
                    } else {
                        Extension::by_id(&state.database, id)
                            .await
                            .map_err(|e| e.into())
                    }
                }
                Err(_) => Extension::by_identifier(&state.database, extension)
                    .await
                    .map_err(|e| e.into()),
            }
        })
        .await
    {
        Ok(Some(extension)) => extension,
        Ok(None) => {
            return Ok(ApiResponse::error("extension not found")
                .with_status(StatusCode::NOT_FOUND)
                .into_response());
        }
        Err(err) => return Ok(ApiResponse::from(err).into_response()),
    };

    req.extensions_mut().insert(user.0);
    req.extensions_mut().insert(extension);

    Ok(next.run(req).await)
}

mod get {
    use crate::{
        response::{ApiResponse, ApiResponseResult},
        routes::{ApiError, GetState, user::admin::extensions::_extension_::GetExtension},
    };
    use serde::Serialize;
    use utoipa::ToSchema;

    #[derive(ToSchema, Serialize)]
    struct Response {
        extension: crate::models::extension::ApiFullExtension,
    }

    #[utoipa::path(get, path = "/", responses(
        (status = OK, body = inline(Response)),
        (status = NOT_FOUND, body = ApiError),
    ), params(
        ("extension" = String, Path, description = "the extension identifier or id")
    ))]
    pub async fn route(state: GetState, extension: GetExtension) -> ApiResponseResult {
        ApiResponse::json(Response {
            extension: extension.0.into_api_full_object(&state.env),
        })
        .ok()
    }
}

mod patch {
    use crate::{
        models::extension::{
            ExtensionPlatform, ExtensionPlatformData, ExtensionType, MinimalExtensionPlatformData,
        },
        response::{ApiResponse, ApiResponseResult},
        routes::{ApiError, GetState, user::admin::extensions::_extension_::GetExtension},
    };
    use serde::{Deserialize, Serialize};
    use std::collections::BTreeMap;
    use utoipa::ToSchema;
    use validator::Validate;

    #[derive(ToSchema, Validate, Deserialize)]
    pub struct Payload {
        #[validate(length(min = 3, max = 63))]
        #[schema(min_length = 3, max_length = 63)]
        name: Option<String>,
        r#type: Option<ExtensionType>,
        unlisted: Option<bool>,
        #[validate(length(min = 3, max = 255))]
        #[schema(min_length = 3, max_length = 255)]
        summary: Option<String>,
        #[validate(length(max = 1024))]
        #[schema(max_length = 1024)]
        description: Option<String>,
        platforms: Option<BTreeMap<ExtensionPlatform, MinimalExtensionPlatformData>>,
    }

    #[derive(ToSchema, Serialize)]
    struct Response {}

    #[utoipa::path(patch, path = "/", responses(
        (status = OK, body = inline(Response)),
        (status = NOT_FOUND, body = ApiError),
    ), params(
        ("extension" = String, Path, description = "the extension identifier or id")
    ), request_body = inline(Payload))]
    pub async fn route(
        state: GetState,
        mut extension: GetExtension,
        axum::Json(data): axum::Json<Payload>,
    ) -> ApiResponseResult {
        if let Some(name) = data.name {
            extension.name = name;
        }
        if let Some(r#type) = data.r#type {
            extension.r#type = r#type;
        }
        if let Some(unlisted) = data.unlisted {
            extension.unlisted = unlisted;
        }
        if let Some(summary) = data.summary {
            extension.summary = summary;
        }
        if let Some(description) = data.description {
            if description.is_empty() {
                extension.description = None;
            } else {
                extension.description = Some(description);
            }
        }
        if let Some(mut platforms) = data.platforms {
            if let Some(builtbybit) = platforms.remove(&ExtensionPlatform::Builtbybit) {
                let url = match reqwest::Url::parse(&builtbybit.url) {
                    Ok(url) => url,
                    Err(_) => return ApiResponse::error("invalid builtbybit url").ok(),
                };

                if url.host_str() != Some("builtbybit.com")
                    || !url.path().starts_with("/resources/")
                {
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

                if let Some(platform) = extension.platforms.get_mut(&ExtensionPlatform::Builtbybit)
                {
                    platform.url = builtbybit.url;
                    platform.price = builtbybit.price;
                    platform.currency = builtbybit.currency;
                } else {
                    extension.platforms.insert(
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
            }

            if let Some(sourcexchange) = platforms.remove(&ExtensionPlatform::Sourcexchange) {
                let url = match reqwest::Url::parse(&sourcexchange.url) {
                    Ok(url) => url,
                    Err(_) => return ApiResponse::error("invalid sourcexchange url").ok(),
                };

                if url.host_str() != Some("www.sourcexchange.net")
                    || !url.path().starts_with("/products/")
                {
                    return ApiResponse::error("invalid sourcexchange url").ok();
                }

                if let Some(platform) = extension
                    .platforms
                    .get_mut(&ExtensionPlatform::Sourcexchange)
                {
                    platform.url = sourcexchange.url;
                    platform.price = sourcexchange.price;
                    platform.currency = sourcexchange.currency;
                } else {
                    extension.platforms.insert(
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
            }

            if let Some(github) = platforms.remove(&ExtensionPlatform::Github) {
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

                if let Some(platform) = extension.platforms.get_mut(&ExtensionPlatform::Github) {
                    platform.url = github.url;
                    platform.price = github.price;
                    platform.currency = github.currency;
                } else {
                    extension.platforms.insert(
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
            }
        }

        sqlx::query!(
            "UPDATE extensions
            SET name = $2, type = $3, unlisted = $4, summary = $5, description = $6, platforms = $7
            WHERE extensions.id = $1",
            extension.id,
            extension.name,
            extension.r#type as ExtensionType,
            extension.unlisted,
            extension.summary,
            extension.description,
            serde_json::to_value(&extension.platforms)?
        )
        .execute(state.database.write())
        .await?;
        state.cache.clear_extension(&extension).await?;

        ApiResponse::json(Response {}).ok()
    }
}

mod delete {
    use crate::{
        response::{ApiResponse, ApiResponseResult},
        routes::{ApiError, GetState, user::admin::extensions::_extension_::GetExtension},
    };
    use serde::Serialize;
    use utoipa::ToSchema;

    #[derive(ToSchema, Serialize)]
    struct Response {}

    #[utoipa::path(delete, path = "/", responses(
        (status = OK, body = inline(Response)),
        (status = NOT_FOUND, body = ApiError),
    ), params(
        ("extension" = String, Path, description = "the extension identifier or id")
    ))]
    pub async fn route(state: GetState, extension: GetExtension) -> ApiResponseResult {
        if extension.banner != "_default.jpeg" {
            tokio::try_join!(
                state
                    .s3
                    .bucket
                    .delete_object(format!("extensions/lowres/{}", extension.banner)),
                state
                    .s3
                    .bucket
                    .delete_object(format!("extensions/{}", extension.banner))
            )?;
        }

        extension.delete(&state.database, &state.s3).await?;
        state.cache.clear_extension(&extension).await?;

        ApiResponse::json(Response {}).ok()
    }
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .routes(routes!(get::route))
        .routes(routes!(patch::route))
        .routes(routes!(delete::route))
        .nest("/ready", approve::router(state))
        .nest("/deny", deny::router(state))
        .nest("/images", images::router(state))
        .nest("/banner", banner::router(state))
        .route_layer(axum::middleware::from_fn_with_state(state.clone(), auth))
        .with_state(state.clone())
}
