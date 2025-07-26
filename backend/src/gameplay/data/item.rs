use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Inventory = HashMap<Item, u32>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Item {
    // Forage
    Mushroom,
    Raspberry,
    Blackberry,
    PineNuts,
    Walnuts,
    DandelionGreens,
    WildGarlic,
    Truffle,

    // Farming
    Wheat,
    Tomato,
    Potato,
    Carrot,
    Onion,
    Turnip,

    // Crafted
    RaspberryJam,
    BlackberryJam,
    RoastedMushrooms,
    BerrySalad,
    NutMix,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    VeryRare,
}

impl Item {
    pub fn price(&self) -> u32 {
        match self {
            // Forage
            Item::Mushroom => 3,
            Item::Raspberry => 2,
            Item::Blackberry => 2,
            Item::PineNuts => 10,
            Item::Walnuts => 10,
            Item::DandelionGreens => 10,
            Item::WildGarlic => 20,
            Item::Truffle => 100,

            // Farming
            Item::Wheat => 5,
            Item::Tomato => 5,
            Item::Potato => 5,
            Item::Carrot => 5,
            Item::Onion => 5,
            Item::Turnip => 5,

            // Crafted
            Item::RaspberryJam => 8,
            Item::BlackberryJam => 8,
            Item::RoastedMushrooms => 8,
            Item::BerrySalad => 20,
            Item::NutMix => 25,
        }
    }

    pub fn rarity(&self) -> Rarity {
        match self {
            // Forage
            Item::Mushroom => Rarity::Common,
            Item::Raspberry => Rarity::Common,
            Item::Blackberry => Rarity::Common,
            Item::PineNuts => Rarity::Uncommon,
            Item::Walnuts => Rarity::Uncommon,
            Item::DandelionGreens => Rarity::Uncommon,
            Item::WildGarlic => Rarity::Rare,
            Item::Truffle => Rarity::VeryRare,

            // Farming
            Item::Wheat => Rarity::Common,
            Item::Tomato => Rarity::Common,
            Item::Potato => Rarity::Common,
            Item::Carrot => Rarity::Common,
            Item::Onion => Rarity::Common,
            Item::Turnip => Rarity::Common,

            // Crafted
            Item::RaspberryJam => Rarity::Common,
            Item::BlackberryJam => Rarity::Common,
            Item::RoastedMushrooms => Rarity::Common,
            Item::BerrySalad => Rarity::Common,
            Item::NutMix => Rarity::Common,
        }
    }
}

pub const FORAGE_TABLE: &[(Item, usize)] = &[
    (Item::Mushroom, 30),
    (Item::Raspberry, 25),
    (Item::Blackberry, 25),
    (Item::PineNuts, 10),
    (Item::DandelionGreens, 10),
    (Item::WildGarlic, 5),
    (Item::Truffle, 1),
];
