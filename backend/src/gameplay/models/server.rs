use crate::gameplay::{
    data::{Item, Recipe},
    models::player::Player,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Server {
    // UUID, Player
    pub players: HashMap<String, Player>,
    pub next_id: u32,
    pub ticks: u32,
}

impl Server {
    // ======================
    // === Server Actions ===
    // ======================

    pub fn new() -> Self {
        Server {
            players: HashMap::new(),
            next_id: 1,
            ticks: 0,
        }
    }

    // ===================
    // === Player Data ===
    // ===================

    pub fn add_player(&mut self, player: Player) {
        self.players.insert(player.uuid.clone(), player);
    }

    pub fn create_player(&mut self, email: String, pw_hash: String, name: String) -> Player {
        let player = Player::new(self.next_id, email, pw_hash, name);
        self.add_player(player.clone());
        self.next_id += 1;
        player
    }

    pub fn _get_player_by_uuid(&self, uuid: &str) -> Result<&Player, String> {
        self.players
            .get(uuid)
            .ok_or_else(|| format!("Player with UUID {} not found", uuid))
    }

    fn get_mut_player_by_uuid(&mut self, uuid: &str) -> Result<&mut Player, String> {
        self.players
            .get_mut(uuid)
            .ok_or_else(|| format!("Player with UUID {} not found", uuid))
    }

    pub fn email_exists(&self, email: &str) -> bool {
        self.players.values().any(|p| p.email == email)
    }

    // ======================
    // === Player Actions ===
    // ======================

    pub fn player_update(
        &mut self,
        uuid: &str,
        name: Option<String>,
        email: Option<String>,
        pw_hash: Option<String>,
    ) -> Result<&Player, String> {
        let player = self.get_mut_player_by_uuid(uuid)?;

        Ok(player.update(name, email, pw_hash))
    }

    pub fn player_forage(&mut self, uuid: &str) -> Result<String, String> {
        let player = self.get_mut_player_by_uuid(uuid)?;

        player.forage()
    }

    pub fn player_craft(&mut self, uuid: &str, recipe: Recipe) -> Result<String, String> {
        let player = self.get_mut_player_by_uuid(uuid)?;

        player.craft(recipe)
    }

    pub fn player_sell(&mut self, uuid: &str, item: Item) -> Result<String, String> {
        let player = self.get_mut_player_by_uuid(uuid)?;

        player.sell(item)
    }
}
