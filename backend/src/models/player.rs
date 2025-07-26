use axum_login::AuthUser;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Player {
    // ID
    pub id: u32,
    pub uuid: String,
    pub email: String,
    pub pw_hash: String,

    // Player Data
    pub name: String,
    pub gold: u32,
    // pub max_energy: u32,
    // pub energy: u32,
    // pub inventory: Inventory,
    // pub farm: Farm,
}

impl Player {
    pub fn new(id: u32, email: String, pw_hash: String, name: String) -> Self {
        Player {
            // ID
            id,
            uuid: Uuid::new_v4().to_string(),
            email,
            pw_hash,

            // Player Data
            name,
            gold: 0,
        }
    }
}
