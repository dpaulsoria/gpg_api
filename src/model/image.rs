use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Image {
    pub id: Uuid,
    pub filename: String,
    pub data: Vec<u8>
}
