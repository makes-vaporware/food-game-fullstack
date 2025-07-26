use crate::{
    auth::AuthSession,
    gameplay::{
        data::Inventory,
        models::{Player, Server},
    },
};
use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use rand_08::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// =========================
// === Individual Routes ===
// =========================

// --- Get Users ---

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    pub id: u32,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct UsersResponse {
    pub users: Vec<UserResponse>,
}

impl From<&Player> for UserResponse {
    fn from(player: &Player) -> Self {
        UserResponse {
            id: player.id,
            name: player.name.clone(),
        }
    }
}

async fn get_all_users(State(state): State<Arc<Mutex<Server>>>) -> impl IntoResponse {
    let server = state.lock().unwrap();

    let users: Vec<UserResponse> = server
        .players
        .values()
        .map(|player| UserResponse::from(player))
        .collect();

    Json(UsersResponse { users }).into_response()
}

// --- Get User by ID ---

async fn get_user_by_id(
    State(state): State<Arc<Mutex<Server>>>,
    Path(id): Path<u32>,
) -> impl IntoResponse {
    let server = state.lock().unwrap();

    let user = server.players.values().find(|player| player.id == id);

    match user {
        Some(player) => {
            let user_response = UserResponse::from(player);
            Json(UsersResponse {
                users: vec![user_response],
            })
            .into_response()
        }
        None => (StatusCode::NOT_FOUND, "User not found").into_response(),
    }
}

// --- Get Me ---

#[derive(Serialize, Deserialize)]
pub struct MeResponse {
    pub id: u32,
    pub uuid: String,
    pub email: String,
    pub name: String,
    pub gold: u32,
    pub energy: u32,
    pub max_energy: u32,
    pub inventory: Inventory,
}

impl From<&Player> for MeResponse {
    fn from(player: &Player) -> Self {
        MeResponse {
            id: player.id,
            uuid: player.uuid.clone(),
            email: player.email.clone(),
            name: player.name.clone(),
            gold: player.gold,
            energy: player.energy,
            max_energy: player.max_energy,
            inventory: player.inventory.clone(),
        }
    }
}

async fn get_me(
    auth_session: AuthSession,
    State(state): State<Arc<Mutex<Server>>>,
) -> impl IntoResponse {
    let user_id = match auth_session.user {
        Some(user) => user.id,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let server = state.lock().unwrap();
    let player = server.players.values().find(|p| p.id == user_id);

    match player {
        Some(player) => {
            let me_response = MeResponse::from(player);
            Json(me_response).into_response()
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

// --- Update Me ---

#[derive(Serialize, Deserialize)]
pub struct UpdateMeRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

async fn update_me(
    mut auth_session: AuthSession,
    State(state): State<Arc<Mutex<Server>>>,
    Json(update_request): Json<UpdateMeRequest>,
) -> impl IntoResponse {
    let user_uuid = match auth_session.user.as_ref() {
        Some(user) => &user.uuid,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let updated_player = {
        let mut server = state.lock().unwrap();

        if let Some(name) = &update_request.name {
            if name.len() < 3 {
                return (
                    StatusCode::BAD_REQUEST,
                    "Player name must be at least 3 characters long",
                )
                    .into_response();
            }
        }

        if let Some(ref email) = update_request.email {
            if !email.contains('@') || !email.contains('.') {
                return (StatusCode::BAD_REQUEST, "Invalid email format").into_response();
            }

            if server
                .players
                .values()
                .any(|p| p.email == *email && p.uuid != *user_uuid)
            {
                return (StatusCode::BAD_REQUEST, "Email already taken").into_response();
            }
        }

        let pw_hash = if let Some(password) = update_request.password {
            if password.len() < 8 {
                return (
                    StatusCode::BAD_REQUEST,
                    "Password must be at least 8 characters long",
                )
                    .into_response();
            }

            let salt = SaltString::generate(&mut OsRng);
            Some(
                Argon2::default()
                    .hash_password(password.as_bytes(), &salt)
                    .expect("Failed to hash password")
                    .to_string(),
            )
        } else {
            None
        };

        server
            .player_update(
                &user_uuid,
                update_request.name,
                update_request.email,
                pw_hash,
            )
            .cloned()
    };

    match updated_player {
        Ok(updated_player) => match auth_session.login(&updated_player).await {
            Ok(_) => (StatusCode::OK, "Updated successfully").into_response(),
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        },
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

// =======================
// === Exported Routes ===
// =======================

pub fn public_routes() -> Router<Arc<Mutex<Server>>> {
    Router::new()
}

pub fn protected_routes() -> Router<Arc<Mutex<Server>>> {
    Router::new()
        .route("/users", get(get_all_users))
        .route("/users/{id}", get(get_user_by_id))
        .route("/me", get(get_me).put(update_me))
}
