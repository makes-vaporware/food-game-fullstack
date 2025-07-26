#[cfg(test)]
mod user_tests {
    use crate::{app::build_app_with_state, gameplay::models::Server};
    use axum::http::StatusCode;
    use axum_test::TestServer;
    use serde_json::{Value, json};
    use std::sync::{Arc, Mutex};

    #[tokio::test]
    async fn get_users() {
        let state = Arc::new(Mutex::new(Server::new()));
        let app = build_app_with_state(state.clone());
        let mut server = TestServer::new(app).unwrap();
        server.save_cookies();

        for i in 1..=3 {
            let sign_up = server
                .post("/sign_up")
                .json(&json!({
                    "email": format!("{}@email.com", i).to_string(),
                    "password": "12345678",
                    "name": format!("Juniper{}", i).to_string(),
                }))
                .await;

            sign_up.assert_status_ok();
        }

        let _sign_in_response = server
            .post("/sign_in")
            .json(&json!({
                "email": "1@email.com",
                "password": "12345678"
            }))
            .await;

        let get_users = server.get("/users").await;

        let json = get_users.json::<Value>();

        let users = json
            .get("users")
            .and_then(|v| v.as_array())
            .expect("Expected a users array");
        assert!(users.len() == 3);

        // Assert users are present
        for i in 1..=3 {
            assert!(users.iter().any(|p| {
                p.get("name").and_then(|n| n.as_str()).expect(&format!(
                    "no name found, p: {:?} and n: {:?}",
                    p,
                    p.get("name")
                )) == format!("Juniper{}", i).to_string()
            }));
        }

        // Ensure no passwords returned
        for user in users.iter() {
            assert!(user.get("pw_hash").is_none());
        }
    }

    #[tokio::test]
    async fn get_user() {
        let state = Arc::new(Mutex::new(Server::new()));
        let app = build_app_with_state(state.clone());
        let mut server = TestServer::new(app).unwrap();
        server.save_cookies();

        for i in 1..=3 {
            let sign_up = server
                .post("/sign_up")
                .json(&json!({
                    "email": format!("{}@email.com", i).to_string(),
                    "password": "12345678",
                    "name": format!("Juniper{}", i).to_string(),
                }))
                .await;

            sign_up.assert_status_ok();
        }

        let _sign_in_response = server
            .post("/sign_in")
            .json(&json!({
                "email": "1@email.com",
                "password": "12345678"
            }))
            .await;

        let get_users = server.get("/users").await;

        let json = get_users.json::<Value>();
        let users = json
            .get("users")
            .and_then(|v| v.as_array())
            .expect("Expected a users array");
        assert!(users.len() == 3);

        for user in users.iter() {
            let id = user.get("id").expect("Expected an id field in player");
            let get_user_by_id = server.get(&format!("/users/{}", id)).await;

            let json_2 = get_user_by_id.json::<Value>();
            let users_2 = json_2
                .get("users")
                .and_then(|v| v.as_array())
                .expect("Expected a users array");
            assert!(users_2.len() == 1);

            assert_eq!(users_2[0], *user);
        }
    }

    // GET /me
    #[tokio::test]
    async fn get_me() {
        let state = Arc::new(Mutex::new(Server::new()));
        let app = build_app_with_state(state.clone());
        let mut server = TestServer::new(app).unwrap();
        server.save_cookies();

        for i in 1..=3 {
            let sign_up = server
                .post("/sign_up")
                .json(&json!({
                    "email": format!("{}@email.com", i).to_string(),
                    "password": "12345678",
                    "name": format!("Juniper{}", i).to_string(),
                }))
                .await;

            sign_up.assert_status_ok();
        }

        let _sign_in_response = server
            .post("/sign_in")
            .json(&json!({
                "email": "1@email.com",
                "password": "12345678"
            }))
            .await;

        let get_me = server.get("/me").await;

        let json = get_me.json::<Value>();

        // Assert right user
        assert_eq!(json.get("email").unwrap(), "1@email.com");
        assert!(json.get("id").is_some());
        assert!(json.get("uuid").is_some());
        assert!(json.get("name").is_some());
        assert!(json.get("gold").is_some());
        assert!(json.get("inventory").is_some());
        assert!(json.get("pw_hash").is_none());
    }

    // PUT /me
    #[tokio::test]
    async fn put_me() {
        let state = Arc::new(Mutex::new(Server::new()));
        let app = build_app_with_state(state.clone());
        let mut server = TestServer::new(app).unwrap();
        server.save_cookies();

        for i in 1..=3 {
            let sign_up = server
                .post("/sign_up")
                .json(&json!({
                    "email": format!("{}@email.com", i).to_string(),
                    "password": "12345678",
                    "name": format!("Juniper{}", i).to_string(),
                }))
                .await;

            sign_up.assert_status_ok();
        }

        let _sign_in_response = server
            .post("/sign_in")
            .json(&json!({
                "email": "1@email.com",
                "password": "12345678"
            }))
            .await;

        let put_me = server
            .put("/me")
            .json(&json!({
                "email": "4@email.com",
                "password": "87654321",
                "name": "Oliver",
            }))
            .await;

        put_me.assert_status_ok();

        let get_me = server.get("/me").await;
        println!("{:?}", get_me);
        let json = get_me.json::<Value>();
        assert_eq!(json.get("email").unwrap(), "4@email.com");
        assert_eq!(json.get("name").unwrap(), "Oliver");

        let _sign_out_response = server.post("/sign_out").await;

        let bad_edited_sign_in = server
            .post("/sign_in")
            .json(&json!({
                "email": "1@email.com",
                "password": "12345678"
            }))
            .await;

        bad_edited_sign_in.assert_status_not_ok();

        let good_edited_sign_in = server
            .post("/sign_in")
            .json(&json!({
                "email": "4@email.com",
                "password": "87654321"
            }))
            .await;

        assert_eq!(good_edited_sign_in.status_code(), 303);
    }
}
