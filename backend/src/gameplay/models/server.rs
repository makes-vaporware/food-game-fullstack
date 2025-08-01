use crate::gameplay::{
    data::{CropType, Item, Recipe},
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

    pub fn advance(&mut self, ticks_elapsed: u32) {
        self.ticks += ticks_elapsed;

        // Restore 1 energy per tick for all players
        // and update their farms
        for (_, player) in &mut self.players {
            player.restore_energy(ticks_elapsed);
            player.update_farm(self.ticks);
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

    pub fn player_plant(
        &mut self,
        uuid: &str,
        plot_id: u32,
        crop_type: CropType,
    ) -> Result<String, String> {
        let current_tick = self.ticks;
        let player = self.get_mut_player_by_uuid(uuid)?;

        player.plant(plot_id, crop_type, current_tick)
    }

    pub fn player_harvest(&mut self, uuid: &str, plot_id: u32) -> Result<String, String> {
        let player = self.get_mut_player_by_uuid(uuid)?;

        player.harvest(plot_id)
    }
}
