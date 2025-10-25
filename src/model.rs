use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub occupation: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    pub name: String,
    pub occupation: String,
}
