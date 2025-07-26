use crate::{auth::backend::AuthBackend, gameplay::models::Server, routes};
use axum::Router;
use axum_login::{
    AuthManagerLayerBuilder,
    tower_sessions::{MemoryStore, SessionManagerLayer},
};
use std::sync::{Arc, Mutex};

pub fn build_app() -> Router {
    // State
    let state = Arc::new(Mutex::new(Server::new()));

    // Session layer.
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store);

    // Auth service.
    let auth_backend = AuthBackend::new(state.clone());
    let auth_layer = AuthManagerLayerBuilder::new(auth_backend, session_layer).build();

    // Build router with auth layer.
    Router::new()
        .merge(routes::public())
        .merge(routes::protected())
        .with_state(state)
        .layer(auth_layer)
}

pub fn build_app_with_state(state: Arc<Mutex<Server>>) -> Router {
    // State given by parameters

    // Session layer.
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store);

    // Auth service - use the same state as the routes
    let auth_backend = AuthBackend::new(state.clone());
    let auth_layer = AuthManagerLayerBuilder::new(auth_backend, session_layer).build();

    // Build router with auth layer.
    Router::new()
        .merge(routes::public())
        .merge(routes::protected())
        .with_state(state)
        .layer(auth_layer)
}
