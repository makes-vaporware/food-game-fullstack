use crate::auth::backend::AuthBackend;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

pub type AuthSession = axum_login::AuthSession<AuthBackend>;
