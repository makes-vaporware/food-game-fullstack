use crate::gameplay::data::{Inventory, Item};

pub fn sell(item: Item, inventory: &Inventory) -> Result<u32, String> {
    if *inventory.get(&item).unwrap_or(&0) == 0 {
        return Err(format!("You don't have any item: {:?}!", item));
    }

    Ok(item.price())
}
