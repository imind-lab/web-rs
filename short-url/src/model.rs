mod url;

pub use url::*;

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow, Clone)]
pub struct Url {
    pub id: String,
    pub url: String,
    pub email: String,
    pub visit: i32,
    pub is_del: bool,
}

#[derive(Deserialize)]
pub struct CreateUrl {
    pub url: String,
    pub email: String,
}
#[derive(Deserialize)]
pub struct UpdateUrl {
    pub id: String,
    pub url: String,
    pub email: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct UrlID {
    pub id: String,
}
#[derive(Debug, Serialize, FromRow)]
pub struct UrlTarget {
    pub url: String,
}
