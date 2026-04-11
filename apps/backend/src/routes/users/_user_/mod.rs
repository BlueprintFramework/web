use std::ops::{Deref, DerefMut};

use crate::routes::{GetState, State};
use crate::{models::user::User, response::ApiResponse};
use axum::{
    extract::{Path, Request},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use utoipa_axum::{router::OpenApiRouter, routes};

mod extensions;

#[derive(Clone)]
pub struct ParamUser(pub User);

impl Deref for ParamUser {
    type Target = User;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ParamUser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub type GetParamUser = crate::extract::ConsumingExtension<ParamUser>;

pub async fn auth(
    state: GetState,
    Path(user): Path<i32>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let user = User::by_id(&state.database, user).await;
    let user = match user {
        Ok(Some(data)) => data,
        Ok(None) => {
            return Ok(ApiResponse::error("user not found")
                .with_status(StatusCode::NOT_FOUND)
                .into_response());
        }
        Err(err) => {
            return Ok(ApiResponse::from(err).into_response());
        }
    };

    req.extensions_mut().insert(ParamUser(user));

    Ok(next.run(req).await)
}

mod get {
    use crate::{
        response::{ApiResponse, ApiResponseResult},
        routes::{GetState, users::_user_::GetParamUser},
    };
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;

    #[derive(ToSchema, Serialize, Deserialize)]
    struct ResponseExtensions {
        total: i64,
    }

    #[derive(ToSchema, Serialize)]
    struct Response {
        user: crate::models::user::ApiUser,

        #[schema(inline)]
        extensions: ResponseExtensions,
    }

    #[utoipa::path(get, path = "/", responses(
        (status = OK, body = inline(Response))
    ), security(
        ("api_key" = [])
    ))]
    pub async fn route(state: GetState, user: GetParamUser) -> ApiResponseResult {
        let extensions = state
            .cache
            .cached(
                &format!("user::{}::extensions_stats_public", user.id),
                600,
                || async {
                    let row = sqlx::query!(
                        "SELECT COUNT(*) AS total
                        FROM extensions
                        WHERE extensions.author_id = $1",
                        user.id
                    )
                    .fetch_one(state.database.read())
                    .await?;

                    Ok(ResponseExtensions {
                        total: row.total.unwrap_or_default(),
                    })
                },
            )
            .await?;

        ApiResponse::new_serialized(Response {
            user: user.0.0.into_api_object(),
            extensions,
        })
        .ok()
    }
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .routes(routes!(get::route))
        .nest("/extensions", extensions::router(state))
        .route_layer(axum::middleware::from_fn_with_state(state.clone(), auth))
        .with_state(state.clone())
}
