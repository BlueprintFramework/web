use super::{ApiError, GetState, State};
use crate::models::author::Author;
use axum::{body::Body, extract::Request, http::StatusCode, middleware::Next, response::Response};
use utoipa_axum::{router::OpenApiRouter, routes};

pub type GetAuthor = axum::extract::Extension<Author>;

async fn auth(state: GetState, mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let key = req
        .headers()
        .get("Authorization")
        .map(|v| v.to_str().unwrap())
        .unwrap_or("")
        .to_string();

    if key.len() != 32 {
        return Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header("Content-Type", "application/json")
            .body(Body::from(
                serde_json::to_string(&ApiError::new(&["invalid authorization header"])).unwrap(),
            ))
            .unwrap());
    }

    let author = Author::by_key(&state.database, &key).await;

    if author.is_none() {
        return Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header("Content-Type", "application/json")
            .body(Body::from(
                serde_json::to_string(&ApiError::new(&["unauthorized"])).unwrap(),
            ))
            .unwrap());
    }

    req.extensions_mut().insert(author.unwrap());

    Ok(next.run(req).await)
}

mod index {
    use super::GetAuthor;
    use crate::{models::author::Author, routes::GetState};
    use serde::Serialize;
    use utoipa::ToSchema;

    #[derive(ToSchema, Serialize)]
    struct Extensions {
        pending: i64,
        approved: i64,
    }

    #[derive(ToSchema, Serialize)]
    struct Response {
        author: Author,

        #[schema(inline)]
        extensions: Extensions,
    }

    #[utoipa::path(get, path = "/", responses(
        (status = OK, body = inline(Response))
    ), security(
        ("api_key" = [])
    ))]
    pub async fn route(state: GetState, author: GetAuthor) -> axum::Json<serde_json::Value> {
        let data = sqlx::query!(
            r#"
            SELECT
                COUNT(*) AS total,
                SUM(pending::int) AS pending
            FROM extensions
            WHERE author_id = $1
            "#,
            author.id
        )
        .fetch_one(state.database.read())
        .await
        .unwrap();

        axum::Json(
            serde_json::to_value(Response {
                author: author.0,
                extensions: Extensions {
                    pending: data.pending.unwrap(),
                    approved: data.total.unwrap() - data.pending.unwrap(),
                },
            })
            .unwrap(),
        )
    }
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .routes(routes!(index::route))
        .route_layer(axum::middleware::from_fn_with_state(state.clone(), auth))
        .with_state(state.clone())
}
