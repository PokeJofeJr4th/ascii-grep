use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CharFile {
    pub rows: Vec<CharEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct CharEntry {
    pub char: char,
    pub description: String,
}
