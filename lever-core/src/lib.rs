use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Feature {
    pub id: Uuid,
    pub name: String,
    pub enabled: bool,
}

impl Feature {
    pub fn new(name: String, enabled: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            enabled,
        }
    }

    pub fn from(id: Uuid, name: String, enabled: bool) -> Self {
        Self {
            id,
            name,
            enabled,
        }
    }
}
