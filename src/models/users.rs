use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
}
