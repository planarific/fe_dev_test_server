use serde::{Deserialize, Serialize};

// returned from /models endpoint
#[derive(Debug, Serialize, Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct ModelListEntry {
    pub id: u32,
    pub description: String,
    pub thumbnail: String,
}

// returned from /models/:id endpoint
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
