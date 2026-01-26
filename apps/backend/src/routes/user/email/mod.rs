use super::State;
use utoipa_axum::{router::OpenApiRouter, routes};

mod verify;

mod patch {
    use crate::{
        response::{ApiResponse, ApiResponseResult},
        routes::{ApiError, GetState, user::GetUser},
    };
    use axum::http::StatusCode;
    use rand::distr::SampleString;
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;
    use validator::Validate;

    #[derive(ToSchema, Validate, Deserialize)]
    pub struct Payload {
        #[validate(email)]
        #[schema(format = "email")]
        email: String,

        captcha: Option<String>,
    }

    #[derive(ToSchema, Serialize)]
    struct Response {}

    #[utoipa::path(patch, path = "/", responses(
        (status = OK, body = inline(Response)),
    ), request_body = inline(Payload))]
    pub async fn route(
        state: GetState,
        ip: crate::GetIp,
        user: GetUser,
        crate::Payload(data): crate::Payload<Payload>,
    ) -> ApiResponseResult {
        if let Err(errors) = crate::utils::validate_data(&data) {
            return ApiResponse::new_serialized(ApiError::new_strings_value(errors))
                .with_status(StatusCode::UNAUTHORIZED)
                .ok();
        }

        if let Err(error) = state.captcha.verify(ip, data.captcha).await {
            return ApiResponse::error(&error)
                .with_status(StatusCode::BAD_REQUEST)
                .ok();
        }

        let email_verification = rand::distr::Alphanumeric.sample_string(&mut rand::rng(), 16);

        sqlx::query!(
            "UPDATE users
            SET email_pending = $2, email_verification = $3
            WHERE users.id = $1",
            user.id,
            data.email,
            email_verification
        )
        .execute(state.database.write())
        .await?;

        let mail = crate::mail::MAIL_VERIFY_EMAIL
            .replace("{{user_name}}", &user.name)
            .replace("{{verification_code}}", &email_verification);

        state.mail.send(
            data.email,
            "Blueprint - Email Verification".to_string(),
            mail,
        );

        ApiResponse::new_serialized(Response {}).ok()
    }
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .routes(routes!(patch::route))
        .nest("/verify", verify::router(state))
        .with_state(state.clone())
}
