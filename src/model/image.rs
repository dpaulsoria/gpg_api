use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Image {
    pub id: usize,
    pub filename: String,
    pub data: Vec<u8>
}
