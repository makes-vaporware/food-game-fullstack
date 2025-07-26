#[cfg(test)]
mod forage_tests {
    use crate::{app::build_app, routes::user::MeResponse};
    use axum_test::TestServer;
    use serde_json::{Value, json};

    async fn sign_up_and_sign_in(server: &TestServer, email: &str, password: &str, name: &str) {
        server
            .post("/sign_up")
            .json(&json!({
                "email": email,
                "password": password,
                "name": name
            }))
            .await;

        server
            .post("/sign_in")
            .json(&json!({
                "email": email,
                "password": password
            }))
            .await;
    }

    #[tokio::test]
    async fn forage() {
        let app = build_app();
        let mut server = TestServer::new(app).unwrap();
        server.save_cookies();

        sign_up_and_sign_in(&server, "1@email.com", "12345678", "Juniper").await;

        let mut num_items = 0;

        for _ in 0..25 {
            let forage = server.post("/forage").await;

            forage.assert_status_ok();

            let inventory = server.get("/me").await.json::<MeResponse>().inventory;

            let forage_json = forage.json::<Value>();
            let message = forage_json.get("message").and_then(|v| v.as_str()).unwrap();
            let success = forage_json
                .get("success")
                .and_then(|v| v.as_bool())
                .unwrap();

            if success {
                if message.contains("Success!") {
                    // Success at foraging item
                    assert_eq!(inventory.values().sum::<u32>(), num_items + 1);
                    num_items += 1
                } else {
                    // No item foraged
                    assert_eq!(inventory.values().sum::<u32>(), num_items);
                }
            } else {
                // Out of energy, end the test.
                break;
            }
        }
    }

    #[tokio::test]
    async fn check_energy() {
        let app = build_app();
        let mut server = TestServer::new(app).unwrap();
        server.save_cookies();

        sign_up_and_sign_in(&server, "1@email.com", "12345678", "Juniper").await;

        // Assuming 25 forage energy
        for _ in 0..25 {
            server.post("/forage").await.assert_status_ok();
        }

        // Out of energy!
        server.post("/forage").await.assert_status_bad_request();
    }
}
