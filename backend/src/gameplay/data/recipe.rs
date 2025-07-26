use serde::{Deserialize, Serialize};

use crate::gameplay::data::{Inventory, Item};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]

pub enum Recipe {
    RaspberryJam,
    BlackberryJam,
    RoastedMushrooms,
    BerrySalad,
    NutMix,
}

impl Recipe {
    pub fn ingredients(&self) -> Inventory {
        match self {
            Recipe::RaspberryJam => HashMap::from([(Item::Raspberry, 3)]),
            Recipe::BlackberryJam => HashMap::from([(Item::Blackberry, 3)]),
            Recipe::RoastedMushrooms => HashMap::from([(Item::Mushroom, 2)]),
            Recipe::BerrySalad => HashMap::from([
                (Item::Raspberry, 1),
                (Item::Blackberry, 1),
                (Item::DandelionGreens, 1),
            ]),
            Recipe::NutMix => HashMap::from([(Item::PineNuts, 1), (Item::Walnuts, 1)]),
        }
    }

    pub fn output(&self) -> Item {
        match self {
            Recipe::RaspberryJam => Item::RaspberryJam,
            Recipe::BlackberryJam => Item::BlackberryJam,
            Recipe::RoastedMushrooms => Item::RoastedMushrooms,
            Recipe::BerrySalad => Item::BerrySalad,
            Recipe::NutMix => Item::NutMix,
        }
    }
}
