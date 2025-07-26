use crate::{
    auth::Credentials,
    gameplay::models::{Player, Server},
};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use std::sync::{Arc, Mutex};

// Authentication setup for Player
impl AuthUser for Player {
    type Id = String;

    fn id(&self) -> Self::Id {
        self.uuid.clone()
    }

    fn session_auth_hash(&self) -> &[u8] {
        &self.pw_hash.as_bytes()
    }
}

// Wrapper for shared Server state
#[derive(Clone)]
pub struct AuthBackend {
    pub server_state: Arc<Mutex<Server>>,
}

impl AuthBackend {
    pub fn new(server_state: Arc<Mutex<Server>>) -> Self {
        Self { server_state }
    }
}

// Authentication setup for AuthBackend (Server by proxy)
#[async_trait]
impl AuthnBackend for AuthBackend {
    type User = Player;
    type Credentials = Credentials;
    type Error = std::convert::Infallible;

    async fn authenticate(
        &self,
        Credentials { email, password }: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let server = self.server_state.lock().unwrap();
        if let Some(player) = server.players.values().find(|p| p.email == email) {
            let parsed_hash = PasswordHash::new(&player.pw_hash).expect("PasswordHash failure");
            if Argon2::default()
                .verify_password(password.as_bytes(), &parsed_hash)
                .is_ok()
            {
                return Ok(Some(player.clone()));
            }
        }

        Ok(None)
    }

    async fn get_user(&self, uuid: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let server = self.server_state.lock().unwrap();
        Ok(server.players.get(uuid).cloned())
    }
}
