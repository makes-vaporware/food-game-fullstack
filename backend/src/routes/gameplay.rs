use crate::{
    auth::AuthSession,
    gameplay::{
        data::{Item, Recipe},
        models::Server,
    },
};
use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// =========================
// === Individual Routes ===
// =========================

// --- Forage ---

#[derive(Serialize, Deserialize)]
pub struct ForageResponse {
    pub message: String,
    pub success: bool,
}

async fn forage(
    auth_session: AuthSession,
    State(state): State<Arc<Mutex<Server>>>,
) -> impl IntoResponse {
    let user_uuid = match auth_session.user.as_ref() {
        Some(user) => &user.uuid,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let mut server = state.lock().unwrap();

    match server.player_forage(user_uuid) {
        Ok(message) => (
            StatusCode::OK,
            Json(ForageResponse {
                message,
                success: true,
            }),
        )
            .into_response(),
        Err(message) => (
            StatusCode::BAD_REQUEST,
            Json(ForageResponse {
                message,
                success: false,
            }),
        )
            .into_response(),
    }
}

// --- Craft ---

#[derive(Serialize, Deserialize)]
pub struct CraftRequest {
    pub recipe: Recipe,
}

#[derive(Serialize, Deserialize)]
pub struct CraftResponse {
    pub message: String,
    pub success: bool,
}

async fn craft(
    auth_session: AuthSession,
    State(state): State<Arc<Mutex<Server>>>,
    Json(craft_request): Json<CraftRequest>,
) -> impl IntoResponse {
    let user_uuid = match auth_session.user.as_ref() {
        Some(user) => &user.uuid,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let mut server = state.lock().unwrap();

    match server.player_craft(user_uuid, craft_request.recipe) {
        Ok(message) => (
            StatusCode::OK,
            Json(CraftResponse {
                message,
                success: true,
            }),
        )
            .into_response(),
        Err(message) => (
            StatusCode::BAD_REQUEST,
            Json(CraftResponse {
                message,
                success: false,
            }),
        )
            .into_response(),
    }
}

// --- Sell ---

#[derive(Serialize, Deserialize)]
pub struct SellRequest {
    pub item: Item,
}

#[derive(Serialize, Deserialize)]
pub struct SellResponse {
    pub message: String,
    pub success: bool,
}

async fn sell(
    auth_session: AuthSession,
    State(state): State<Arc<Mutex<Server>>>,
    Json(sell_request): Json<SellRequest>,
) -> impl IntoResponse {
    let user_uuid = match auth_session.user.as_ref() {
        Some(user) => &user.uuid,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let mut server = state.lock().unwrap();

    match server.player_sell(user_uuid, sell_request.item) {
        Ok(message) => (
            StatusCode::OK,
            Json(SellResponse {
                message,
                success: true,
            }),
        )
            .into_response(),
        Err(message) => (
            StatusCode::BAD_REQUEST,
            Json(SellResponse {
                message,
                success: false,
            }),
        )
            .into_response(),
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
        .route("/forage", post(forage))
        .route("/craft", post(craft))
        .route("/sell", post(sell))
}
