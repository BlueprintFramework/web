use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sqlx::postgres::PgRow;
use std::collections::BTreeMap;
use utoipa::ToSchema;
use validator::Validate;

pub mod extension;
pub mod user;
pub mod user_session;

#[derive(ToSchema, Validate, Deserialize)]
pub struct PaginationParams {
    #[validate(range(min = 1))]
    #[serde(default = "Pagination::default_page")]
    pub page: i64,
    #[validate(range(min = 1, max = 100))]
    #[serde(default = "Pagination::default_per_page")]
    pub per_page: i64,
}

#[derive(ToSchema, Serialize)]
pub struct Pagination<T: Serialize = serde_json::Value> {
    pub total: i64,
    pub per_page: i64,
    pub page: i64,

    pub data: Vec<T>,
}

impl Pagination {
    pub const fn default_page() -> i64 {
        1
    }

    pub const fn default_per_page() -> i64 {
        25
    }
}

pub trait BaseModel: Serialize + DeserializeOwned {
    fn columns(prefix: Option<&str>, table: Option<&str>) -> BTreeMap<String, String>;

    #[inline]
    fn columns_sql(prefix: Option<&str>, table: Option<&str>) -> String {
        Self::columns(prefix, table)
            .iter()
            .map(|(key, value)| format!("{key} as {value}"))
            .collect::<Vec<String>>()
            .join(", ")
    }

    fn map(prefix: Option<&str>, row: &PgRow) -> Self;
}
