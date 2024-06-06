use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: usize,
    pub username: String,
    pub password: String,
    pub email: String,
}
