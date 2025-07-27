use crate::{
    auth::{AuthSession, Credentials},
    gameplay::models::Server,
    routes::{json_error, json_success},
};
use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
};
use rand_08::rngs::OsRng;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::{Arc, Mutex};

// =========================
// === Individual Routes ===
// =========================

// --- Root ---

async fn root() -> impl IntoResponse {
    (StatusCode::OK, "Hello World!").into_response()
}

// --- Sign Up ---

#[derive(Serialize, Deserialize)]
pub struct SignUpForm {
    email: String,
    password: String,
    name: String,
}

async fn sign_up(
    State(state): State<Arc<Mutex<Server>>>,
    Json(form): Json<SignUpForm>,
) -> impl IntoResponse {
    let mut server = state.lock().unwrap();
    let SignUpForm {
        email,
        password,
        name,
    } = form;

    if server.email_exists(&email) {
        return (StatusCode::BAD_REQUEST, json_error("Email already exists")).into_response();
    }

    if !email.contains('@') || !email.contains('.') {
        return (StatusCode::BAD_REQUEST, json_error("Invalid email format")).into_response();
    }

    if password.len() < 8 {
        return (
            StatusCode::BAD_REQUEST,
            json_error("Password must be at least 8 characters long"),
        )
            .into_response();
    }

    if name.len() < 3 {
        return (
            StatusCode::BAD_REQUEST,
            json_error("Player name must be at least 3 characters long"),
        )
            .into_response();
    }

    let salt = SaltString::generate(&mut OsRng);
    let pw_hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string();

    server.create_player(email, pw_hash, name);

    (StatusCode::OK, json_success("Account created")).into_response()
}

// --- Sign In ---

async fn sign_in(
    mut auth_session: AuthSession,
    Json(creds): Json<Credentials>,
) -> impl IntoResponse {
    let player = match auth_session.authenticate(creds.clone()).await {
        Ok(Some(player)) => player,
        Ok(None) => return StatusCode::UNAUTHORIZED.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    match auth_session.login(&player).await {
        Ok(_) => Redirect::to("/dashboard").into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

// --- Sign Out ---

async fn sign_out(mut auth_session: AuthSession) -> impl IntoResponse {
    match auth_session.logout().await {
        Ok(_) => Redirect::to("/").into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

// =======================
// === Exported Routes ===
// =======================

pub fn public_routes() -> Router<Arc<Mutex<Server>>> {
    Router::new()
        .route("/", get(root))
        .route("/sign_up", post(sign_up))
        .route("/sign_in", post(sign_in))
}

pub fn protected_routes() -> Router<Arc<Mutex<Server>>> {
    Router::new().route("/sign_out", post(sign_out))
}
