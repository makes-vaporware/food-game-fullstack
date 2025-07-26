// #[cfg(test)]
// mod craft_tests {
//     use crate::app::build_app;
//     use axum_test::TestServer;
//     use serde_json::{Value, json};
//     use std::collections::HashMap;

//     #[tokio::test]
//     async fn craft_simple() {
//         let app = build_app();
//         let mut server = TestServer::new(app).unwrap();
//         server.save_cookies();

//         let _sign_up_response = server
//             .post("/sign_up")
//             .json(&json!({
//                 "email": "1@email.com",
//                 "password": "12345678",
//                 "player_name": "Juniper",
//             }))
//             .await;

//         let _sign_in_response = server
//             .post("/sign_in")
//             .json(&json!({
//                 "email": "1@email.com",
//                 "password": "12345678"
//             }))
//             .await;

//         let mut player = Player::new(String::from("Juniper"));
//         assert!(player.inventory.is_empty());

//         player.inventory = HashMap::from([
//             (Item::Raspberry, 4),
//             (Item::Blackberry, 4),
//             (Item::DandelionGreens, 1),
//         ]);

//         // Craft RaspberryJam
//         assert_ok!(player.craft(Recipe::RaspberryJam));
//         assert_eq!(
//             player.inventory,
//             HashMap::from([
//                 (Item::Raspberry, 1),
//                 (Item::Blackberry, 4),
//                 (Item::DandelionGreens, 1),
//                 (Item::RaspberryJam, 1)
//             ])
//         );
//     }

//     #[test]
//     fn craft_extended() {
//         let mut server = Server::new();
//         let id = server.create_player(String::from("Juniper"));

//         let player = server.get_mut_player(id).unwrap();
//         assert!(player.inventory.is_empty());

//         player.inventory = HashMap::from([
//             (Item::Raspberry, 4),
//             (Item::Blackberry, 4),
//             (Item::DandelionGreens, 1),
//         ]);

//         // Craft RaspberryJam
//         assert_ok!(server.player_craft(id, Recipe::RaspberryJam));

//         assert_player_attribute_eq!(
//             server,
//             id,
//             inventory,
//             HashMap::from([
//                 (Item::Raspberry, 1),
//                 (Item::Blackberry, 4),
//                 (Item::DandelionGreens, 1),
//                 (Item::RaspberryJam, 1)
//             ])
//         );

//         // Craft BlackberryJam
//         // assert_ok!(player.craft(Recipe::BlackberryJam));
//         assert_ok!(server.player_craft(id, Recipe::BlackberryJam));

//         assert_player_attribute_eq!(
//             server,
//             id,
//             inventory,
//             HashMap::from([
//                 (Item::Raspberry, 1),
//                 (Item::Blackberry, 1),
//                 (Item::DandelionGreens, 1),
//                 (Item::RaspberryJam, 1),
//                 (Item::BlackberryJam, 1)
//             ])
//         );

//         // Craft BerrySalad
//         assert_ok!(server.player_craft(id, Recipe::BerrySalad));
//         assert_player_attribute_eq!(
//             server,
//             id,
//             inventory,
//             HashMap::from([
//                 (Item::RaspberryJam, 1),
//                 (Item::BlackberryJam, 1),
//                 (Item::BerrySalad, 1)
//             ])
//         );

//         // Sell crafted items
//         assert_player_attribute_eq!(server, id, gold, 0);

//         assert_ok!(server.player_sell(id, Item::RaspberryJam));
//         assert_ok!(server.player_sell(id, Item::BlackberryJam));
//         assert_ok!(server.player_sell(id, Item::BerrySalad));

//         assert_player_attribute_eq!(
//             server,
//             id,
//             gold,
//             Item::RaspberryJam.price() + Item::BlackberryJam.price() + Item::BerrySalad.price()
//         );
//     }

//     #[test]
//     fn cant_craft_without_ingredients() {
//         let mut server = Server::new();
//         let id = server.create_player(String::from("Juniper"));

//         let player = server.get_player(id).unwrap();
//         assert!(player.inventory.is_empty());

//         assert_err!(server.player_craft(id, Recipe::BerrySalad));
//     }
// }
