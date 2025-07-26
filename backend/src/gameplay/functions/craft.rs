use crate::gameplay::data::{Inventory, Item, Recipe};

pub fn craft(recipe: Recipe, inventory: &Inventory) -> Result<(Item, Inventory), String> {
    for (ingredient, count) in recipe.ingredients() {
        let available = *inventory.get(&ingredient).unwrap_or(&0);

        if available < count {
            return Err(format!(
                "Not enough {:?}! Need {}, have {}",
                ingredient, count, available
            ));
        }
    }

    Ok((recipe.output(), recipe.ingredients()))
}
