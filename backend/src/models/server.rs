use argon2::{Argon2, PasswordHash, PasswordVerifier};
use async_trait::async_trait;
use axum_login::{AuthnBackend, UserId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::models::player::Player;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Server {
    pub players: HashMap<String, Player>,
    pub next_id: u32,
    pub ticks: u32,
}

impl Server {
    pub fn new() -> Self {
        Server {
            players: HashMap::new(),
            next_id: 1,
            ticks: 0,
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.insert(player.email.clone(), player);
    }
}

// #[derive(Clone)]
// pub struct Credentials {
//     email: String,
//     password: String,
// }

// #[async_trait]
// impl AuthnBackend for Server {
//     type User = Player;
//     type Credentials = Credentials;
//     type Error = std::convert::Infallible;

//     async fn authenticate(
//         &self,
//         Credentials { email, password }: Self::Credentials,
//     ) -> Result<Option<Self::User>, Self::Error> {
//         if let Some(player) = self.players.values().find(|p| p.email == email) {
//             let parsed_hash = PasswordHash::new(&player.pw_hash).expect("PasswordHash failure");
//             if Argon2::default()
//                 .verify_password(password.as_bytes(), &parsed_hash)
//                 .is_ok()
//             {
//                 return Ok(Some(player.clone()));
//             }
//         }

//         Ok(None)
//     }

//     async fn get_user(&self, uuid: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
//         Ok(self.players.get(uuid).cloned())
//     }
// }
