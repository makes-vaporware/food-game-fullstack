use crate::{auth::backend::AuthBackend, gameplay::models::Server};
use axum::Router;
use axum_login::login_required;
use std::sync::{Arc, Mutex};

pub mod auth;
pub mod gameplay;
pub mod user;

pub fn public() -> Router<Arc<Mutex<Server>>> {
    Router::new()
        .merge(auth::public_routes())
        .merge(gameplay::public_routes())
        .merge(user::public_routes())
}

pub fn protected() -> Router<Arc<Mutex<Server>>> {
    Router::new()
        .merge(auth::protected_routes())
        .merge(gameplay::protected_routes())
        .merge(user::protected_routes())
        .route_layer(login_required!(AuthBackend, login_url = "/sign_in"))
}
