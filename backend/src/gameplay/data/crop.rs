use crate::gameplay::data::Item;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CropType {
    Wheat,
    Tomato,
    Potato,
    Carrot,
    Onion,
    Turnip,
}

impl CropType {
    pub fn grow_time(&self) -> u32 {
        match self {
            CropType::Wheat => 10,
            CropType::Tomato => 15,
            CropType::Potato => 30,
            CropType::Carrot => 10,
            CropType::Onion => 15,
            CropType::Turnip => 20,
        }
    }

    pub fn to_item(&self) -> Item {
        match self {
            CropType::Wheat => Item::Wheat,
            CropType::Tomato => Item::Tomato,
            CropType::Potato => Item::Potato,
            CropType::Carrot => Item::Carrot,
            CropType::Onion => Item::Onion,
            CropType::Turnip => Item::Turnip,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Crop {
    pub kind: CropType,
    pub planted_at: u32,
    pub grow_time: u32,
    pub is_ready: bool,
}

impl Crop {
    pub fn new(kind: CropType, current_tick: u32) -> Self {
        let grow_time = kind.grow_time();
        Crop {
            kind,
            planted_at: current_tick,
            grow_time,
            is_ready: false,
        }
    }

    pub fn update(&mut self, current_tick: u32) {
        if !self.is_ready && current_tick >= self.planted_at + self.grow_time {
            self.is_ready = true;
        }
    }

    pub fn harvest(self) -> Item {
        self.kind.to_item()
    }
}
