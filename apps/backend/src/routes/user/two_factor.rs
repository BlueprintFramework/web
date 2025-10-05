use super::State;
use utoipa_axum::{router::OpenApiRouter, routes};

mod get {
    use crate::{
        response::{ApiResponse, ApiResponseResult},
        routes::{ApiError, GetState, user::GetUser},
    };
    use axum::http::StatusCode;
    use serde::Serialize;
    use utoipa::ToSchema;

    #[derive(ToSchema, Serialize)]
    struct Response {
        otp_url: String,
        secret: String,
    }

    #[utoipa::path(get, path = "/", responses(
        (status = OK, body = inline(Response)),
        (status = CONFLICT, body = ApiError),
    ))]
    pub async fn route(state: GetState, user: GetUser) -> ApiResponseResult {
        if user.totp_enabled {
            return ApiResponse::error("two-factor authentication is already enabled")
                .with_status(StatusCode::CONFLICT)
                .ok();
        }

        let secret = match totp_rs::Secret::generate_secret().to_encoded() {
            totp_rs::Secret::Encoded(secret) => secret,
            _ => unreachable!(),
        };

        sqlx::query!(
            "UPDATE users
            SET totp_secret = $1
            WHERE users.id = $2",
            secret,
            user.id
        )
        .execute(state.database.write())
        .await?;

        ApiResponse::json(Response {
            otp_url: format!(
                "otpauth://totp/{name}:{}?secret={}&issuer={name}",
                urlencoding::encode(&user.email),
                urlencoding::encode(&secret),
                name = urlencoding::encode("Blueprint")
            ),
            secret,
        })
        .ok()
    }
}

mod post {
    use crate::{
        models::user_recovery_code::UserRecoveryCode,
        response::{ApiResponse, ApiResponseResult},
        routes::{ApiError, GetState, user::GetUser},
    };
    use axum::http::StatusCode;
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;
    use validator::Validate;

    #[derive(ToSchema, Validate, Deserialize)]
    pub struct Payload {
        #[validate(length(equal = 6))]
        #[schema(min_length = 6, max_length = 6)]
        code: String,
        #[validate(length(max = 512))]
        #[schema(max_length = 512)]
        password: String,
    }

    #[derive(ToSchema, Serialize)]
    struct Response {
        recovery_codes: Vec<String>,
    }

    #[utoipa::path(post, path = "/", responses(
        (status = OK, body = inline(Response)),
        (status = BAD_REQUEST, body = ApiError),
        (status = CONFLICT, body = ApiError),
        (status = UNAUTHORIZED, body = ApiError),
    ), request_body = inline(Payload))]
    pub async fn route(
        state: GetState,
        user: GetUser,
        axum::Json(data): axum::Json<Payload>,
    ) -> ApiResponseResult {
        if user.totp_enabled {
            return ApiResponse::error("two-factor authentication is already enabled")
                .with_status(StatusCode::CONFLICT)
                .ok();
        }

        let totp_secret = match &user.totp_secret {
            Some(secret) => secret,
            None => {
                return ApiResponse::error("two-factor authentication has not been configured")
                    .with_status(StatusCode::UNAUTHORIZED)
                    .ok();
            }
        };

        if let Err(errors) = crate::utils::validate_data(&data) {
            return ApiResponse::json(ApiError::new_strings_value(errors))
                .with_status(StatusCode::BAD_REQUEST)
                .ok();
        }

        if !user
            .validate_password(&state.database, &data.password)
            .await?
        {
            return ApiResponse::error("invalid password")
                .with_status(StatusCode::UNAUTHORIZED)
                .ok();
        }

        let totp = totp_rs::TOTP::new(
            totp_rs::Algorithm::SHA1,
            6,
            1,
            30,
            totp_rs::Secret::Encoded(totp_secret.clone()).to_bytes()?,
        )?;

        if !totp.check_current(&data.code).is_ok_and(|valid| valid) {
            return ApiResponse::error("invalid confirmation code")
                .with_status(StatusCode::BAD_REQUEST)
                .ok();
        }

        let recovery_codes = UserRecoveryCode::create_all(&state.database, user.id).await?;

        sqlx::query!(
            "UPDATE users
            SET totp_enabled = true
            WHERE users.id = $1",
            user.id
        )
        .execute(state.database.write())
        .await?;

        ApiResponse::json(Response { recovery_codes }).ok()
    }
}

mod delete {
    use crate::{
        models::user_recovery_code::UserRecoveryCode,
        response::{ApiResponse, ApiResponseResult},
        routes::{ApiError, GetState, user::GetUser},
    };
    use axum::http::StatusCode;
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;
    use validator::Validate;

    #[derive(ToSchema, Validate, Deserialize)]
    pub struct Payload {
        #[validate(length(min = 6, max = 10))]
        #[schema(min_length = 6, max_length = 10)]
        code: String,
        #[validate(length(max = 512))]
        #[schema(max_length = 512)]
        password: String,
    }

    #[derive(ToSchema, Serialize)]
    struct Response {}

    #[utoipa::path(delete, path = "/", responses(
        (status = OK, body = inline(Response)),
        (status = BAD_REQUEST, body = ApiError),
        (status = CONFLICT, body = ApiError),
        (status = UNAUTHORIZED, body = ApiError),
    ), request_body = inline(Payload))]
    pub async fn route(
        state: GetState,
        mut user: GetUser,
        axum::Json(data): axum::Json<Payload>,
    ) -> ApiResponseResult {
        if !user.totp_enabled {
            return ApiResponse::error("two-factor authentication is not enabled")
                .with_status(StatusCode::CONFLICT)
                .ok();
        }

        if let Err(errors) = crate::utils::validate_data(&data) {
            return ApiResponse::json(ApiError::new_strings_value(errors))
                .with_status(StatusCode::BAD_REQUEST)
                .ok();
        }

        if !user
            .validate_password(&state.database, &data.password)
            .await?
        {
            return ApiResponse::error("invalid password")
                .with_status(StatusCode::UNAUTHORIZED)
                .ok();
        }

        match data.code.len() {
            6 => {
                let totp = totp_rs::TOTP::new(
                    totp_rs::Algorithm::SHA1,
                    6,
                    1,
                    30,
                    totp_rs::Secret::Encoded(user.0.totp_secret.take().unwrap()).to_bytes()?,
                )?;

                if !totp.check_current(&data.code).is_ok_and(|valid| valid) {
                    return ApiResponse::error("invalid confirmation code")
                        .with_status(StatusCode::BAD_REQUEST)
                        .ok();
                }
            }
            10 => {
                if UserRecoveryCode::delete_by_user_id_code(&state.database, user.id, &data.code)
                    .await?
                    .is_none()
                {
                    return ApiResponse::error("invalid recovery code")
                        .with_status(StatusCode::BAD_REQUEST)
                        .ok();
                }
            }
            _ => {
                return ApiResponse::error("invalid confirmation code length")
                    .with_status(StatusCode::BAD_REQUEST)
                    .ok();
            }
        }

        UserRecoveryCode::delete_by_user_id(&state.database, user.id).await?;

        sqlx::query!(
            "UPDATE users
            SET totp_enabled = false, totp_secret = NULL
            WHERE users.id = $1",
            user.id
        )
        .execute(state.database.write())
        .await?;

        ApiResponse::json(Response {}).ok()
    }
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .routes(routes!(get::route))
        .routes(routes!(post::route))
        .routes(routes!(delete::route))
        .with_state(state.clone())
}
