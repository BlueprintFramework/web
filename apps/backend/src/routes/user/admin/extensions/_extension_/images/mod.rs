use super::State;
use axum::extract::DefaultBodyLimit;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

mod _image_;

mod get {
    use crate::{
        models::{PaginationParams, extension_image::ExtensionImage},
        response::{ApiResponse, ApiResponseResult},
        routes::{ApiError, GetState, user::admin::extensions::_extension_::GetExtension},
    };
    use axum::{extract::Query, http::StatusCode};
    use serde::Serialize;
    use utoipa::ToSchema;

    #[derive(ToSchema, Serialize)]
    struct Response {
        extension_images: Vec<crate::models::extension_image::ApiExtensionImage>,
    }

    #[utoipa::path(get, path = "/", responses(
        (status = OK, body = inline(Response)),
    ), params(
        ("extension" = String, Path, description = "the extension identifier or id")
    ))]
    pub async fn route(
        state: GetState,
        extension: GetExtension,
        Query(params): Query<PaginationParams>,
    ) -> ApiResponseResult {
        if let Err(errors) = crate::utils::validate_data(&params) {
            return ApiResponse::json(ApiError::new_strings_value(errors))
                .with_status(StatusCode::UNAUTHORIZED)
                .ok();
        }

        let extension_images =
            ExtensionImage::all_by_extension_id(&state.database, extension.id).await?;

        ApiResponse::json(Response {
            extension_images: extension_images
                .into_iter()
                .map(|extension_image| extension_image.into_api_object(&state.env))
                .collect(),
        })
        .ok()
    }
}

mod post {
    use crate::{
        models::extension_image::ExtensionImage,
        response::{ApiResponse, ApiResponseResult},
        routes::{GetState, user::admin::extensions::_extension_::GetExtension},
    };
    use axum::{body::Bytes, http::StatusCode};
    use image::ImageReader;
    use serde::Serialize;
    use utoipa::ToSchema;

    #[derive(ToSchema, Serialize)]
    struct Response {
        extension_image: crate::models::extension_image::ApiExtensionImage,
    }

    #[utoipa::path(post, path = "/", responses(
        (status = OK, body = inline(Response)),
    ), params(
        ("extension" = String, Path, description = "the extension identifier or id")
    ), request_body = String)]
    pub async fn route(
        state: GetState,
        extension: GetExtension,
        image: Bytes,
    ) -> ApiResponseResult {
        let image = match ImageReader::new(std::io::Cursor::new(image)).with_guessed_format() {
            Ok(reader) => reader,
            Err(_) => {
                return ApiResponse::error("image: unable to decode")
                    .with_status(StatusCode::BAD_REQUEST)
                    .ok();
            }
        };

        let image = match tokio::task::spawn_blocking(move || image.decode()).await? {
            Ok(image) => image,
            Err(_) => {
                return ApiResponse::error("image: unable to decode")
                    .with_status(StatusCode::BAD_REQUEST)
                    .ok();
            }
        };

        let extension_image =
            ExtensionImage::create(&state.database, &state.s3, &extension, image).await?;

        ApiResponse::json(Response {
            extension_image: extension_image.into_api_object(&state.env),
        })
        .ok()
    }
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .routes(routes!(get::route))
        .routes(routes!(post::route).layer(DefaultBodyLimit::max(8 * 1024 * 1024)))
        .nest("/{image}", _image_::router(state))
        .with_state(state.clone())
}
