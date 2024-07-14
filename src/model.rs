use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct Model {
    pub id: u32,
    pub address1: String,
    pub address2: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub thumbnail: String,
    pub model: String,
}
