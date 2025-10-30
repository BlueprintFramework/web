use super::State;
use axum::extract::DefaultBodyLimit;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

mod post {
    use crate::{
        response::{ApiResponse, ApiResponseResult},
        routes::{ApiError, GetState, user::admin::extensions::_extension_::GetExtension},
    };
    use axum::{body::Bytes, http::StatusCode};
    use image::{ImageReader, codecs::jpeg::JpegEncoder, imageops::FilterType};
    use rand::distr::SampleString;
    use serde::Serialize;
    use utoipa::ToSchema;

    #[derive(ToSchema, Serialize)]
    struct Response {}

    #[utoipa::path(post, path = "/", responses(
        (status = OK, body = inline(Response)),
        (status = NOT_FOUND, body = ApiError),
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

        let (lowres_data, fullres_data) = tokio::task::spawn_blocking(
            move || -> Result<(Vec<u8>, Vec<u8>), image::ImageError> {
                let lowres_image = image.resize_exact(1280, 720, FilterType::Triangle);
                let mut lowres_data: Vec<u8> = Vec::new();
                let mut encoder = JpegEncoder::new(&mut lowres_data);
                let color = lowres_image.color();
                encoder.encode(lowres_image.as_bytes(), 1280, 720, color.into())?;

                let fullres_image = image.resize_exact(1920, 1080, FilterType::Triangle);
                let mut fullres_data: Vec<u8> = Vec::new();
                let mut encoder = JpegEncoder::new(&mut fullres_data);
                let color = fullres_image.color();
                encoder.encode(fullres_image.as_bytes(), 1920, 1080, color.into())?;

                Ok((lowres_data, fullres_data))
            },
        )
        .await??;

        let identifier_random = rand::distr::Alphanumeric.sample_string(&mut rand::rng(), 8);
        let banner_name = format!("{}-{}.jpeg", extension.identifier, identifier_random);

        tokio::try_join!(
            state.s3.upload(
                format!("extensions/lowres/{banner_name}"),
                &lowres_data,
                Some("image/jpeg")
            ),
            state.s3.upload(
                format!("extensions/{banner_name}"),
                &fullres_data,
                Some("image/jpeg")
            ),
            async {
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

                Ok(())
            }
        )?;

        sqlx::query!(
            "UPDATE extensions
            SET banner = $2
            WHERE extensions.id = $1",
            extension.id,
            banner_name
        )
        .execute(state.database.write())
        .await?;
        state.cache.clear_extension(&extension).await?;

        ApiResponse::json(Response {}).ok()
    }
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .routes(routes!(post::route).layer(DefaultBodyLimit::max(8 * 1024 * 1024)))
        .with_state(state.clone())
}
