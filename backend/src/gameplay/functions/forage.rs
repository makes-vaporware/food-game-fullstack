use crate::gameplay::data::{FORAGE_TABLE, Item};
use rand::{Rng, seq::IndexedRandom};

pub fn forage() -> Result<(Option<Item>, String), String> {
    let mut rng = rand::rng();
    let result = rng.random::<f32>();

    if result < 0.5 {
        let messages = vec![
            "You find only broken twigs and deer tracks.",
            "Something rustled nearby, but you couldn't find it.",
            "Nothing but dirt and disappointment.",
            "You find a nice meadow and stop to catch your breath.",
            "Your path was blocked by a herd of geese. You decided to leave quietly.",
            "You picked up some litter along the path.",
            "The river is cold today.",
        ];

        Ok((None, format!("{}", messages.choose(&mut rng).unwrap())))
    } else {
        let item = FORAGE_TABLE
            .choose_weighted(&mut rng, |ingredient| ingredient.1)
            .unwrap()
            .0;

        Ok((Some(item), format!("Success! You found a: {:?}", item)))
    }
}
