use crate::gameplay::data::{Inventory, Item, Recipe};
use crate::gameplay::functions::{craft, forage, sell};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Player {
    // ID
    pub id: u32,
    pub uuid: String,
    pub email: String,
    pub pw_hash: String,

    // Player Data
    pub name: String,
    pub gold: u32,
    pub max_energy: u32,
    pub energy: u32,
    pub inventory: Inventory,
    // pub farm: Farm,
}

impl Player {
    pub fn new(id: u32, email: String, pw_hash: String, name: String) -> Self {
        Player {
            // ID
            id,
            uuid: Uuid::new_v4().to_string(),
            email,
            pw_hash,

            // Player Data
            name,
            gold: 0,
            max_energy: 25,
            energy: 25,
            inventory: HashMap::new(),
        }
    }

    pub fn update(
        &mut self,
        name: Option<String>,
        email: Option<String>,
        pw_hash: Option<String>,
    ) -> &Player {
        if let Some(new_name) = name {
            self.name = new_name;
        }

        if let Some(new_email) = email {
            self.email = new_email;
        }

        if let Some(new_pw_hash) = pw_hash {
            self.pw_hash = new_pw_hash;
        }

        self
    }

    pub fn forage(&mut self) -> Result<String, String> {
        if self.energy == 0 {
            return Err(format!(
                "You're out of energy! (Energy: 0/{})",
                self.max_energy
            ));
        }

        self.energy -= 1;
        let (res, msg) = forage()?;

        match res {
            Some(item) => *self.inventory.entry(item).or_insert(0) += 1,
            None => {}
        }

        Ok(format!(
            "{} (Energy: {}/{})",
            msg, self.energy, self.max_energy
        ))
    }

    pub fn restore_energy(&mut self, amount: u32) {
        self.energy = (self.energy + amount).min(self.max_energy);
    }

    pub fn craft(&mut self, recipe: Recipe) -> Result<String, String> {
        let (crafted, to_deduct) = craft(recipe, &self.inventory)?;

        for (ingredient, count) in to_deduct {
            println!("{:?} {:?} {}", self.inventory, ingredient, count);

            let entry = self.inventory.get_mut(&ingredient).unwrap();
            *entry -= count;

            if *entry == 0 {
                self.inventory.remove(&ingredient);
            }
        }

        *self.inventory.entry(crafted).or_insert(0) += 1;

        Ok(format!("Crafted {:?}!", crafted))
    }

    pub fn sell(&mut self, item: Item) -> Result<String, String> {
        let profit = sell(item, &self.inventory)?;

        let entry = self.inventory.get_mut(&item).unwrap();
        *entry -= 1;

        if *entry == 0 {
            self.inventory.remove(&item);
        }

        self.gold += profit;

        Ok(format!("Sold {:?} for {} gold!", item, profit))
    }
}
