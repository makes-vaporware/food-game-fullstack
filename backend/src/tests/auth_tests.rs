#[cfg(test)]
mod auth_tests {
    use crate::{app::build_app_with_state, gameplay::models::Server};
    use axum_test::TestServer;
    use serde_json::json;
    use std::sync::{Arc, Mutex};

    #[tokio::test]
    async fn sign_up_simple() {
        let state = Arc::new(Mutex::new(Server::new()));
        let app = build_app_with_state(state.clone());
        let server = TestServer::new(app).unwrap();

        let response = server
            .post("/sign_up")
            .json(&json!({
                "email": "1@email.com",
                "password": "12345678",
                "name": "Juniper",
            }))
            .await;

        response.assert_status_ok();

        let server_lock = state.lock().unwrap();
        let player = server_lock
            .players
            .values()
            .find(|p| p.email == "1@email.com")
            .expect("Player should exist");

        assert_eq!(player.name, "Juniper");
    }

    #[tokio::test]
    async fn sign_up_validation_checks() {
        let state = Arc::new(Mutex::new(Server::new()));
        let app = build_app_with_state(state.clone());
        let server = TestServer::new(app).unwrap();

        // Invalid email format
        let bad_email = server
            .post("/sign_up")
            .json(&json!({
                "email": "not an email",
                "password": "12345678",
                "name": "Juniper",
            }))
            .await;

        bad_email.assert_status_bad_request();

        // Password length < 8
        let short_password = server
            .post("/sign_up")
            .json(&json!({
                "email": "1@email.com",
                "password": "1234",
                "name": "Juniper",
            }))
            .await;

        short_password.assert_status_bad_request();

        // Repeated email usage
        let response = server
            .post("/sign_up")
            .json(&json!({
                "email": "1@email.com",
                "password": "12345678",
                "name": "Juniper",
            }))
            .await;

        response.assert_status_ok();

        let repeat_email = server
            .post("/sign_up")
            .json(&json!({
                "email": "1@email.com",
                "password": "12345678",
                "name": "Juniper",
            }))
            .await;

        repeat_email.assert_status_bad_request();
    }

    #[tokio::test]
    async fn sign_in_simple() {
        let state = Arc::new(Mutex::new(Server::new()));
        let app = build_app_with_state(state.clone());
        let server = TestServer::new(app).unwrap();

        let sign_up_response = server
            .post("/sign_up")
            .json(&json!({
                "email": "1@email.com",
                "password": "12345678",
                "name": "Juniper",
            }))
            .await;

        sign_up_response.assert_status_ok();

        let sign_in_response = server
            .post("/sign_in")
            .json(&json!({
                "email": "1@email.com",
                "password": "12345678"
            }))
            .await;

        assert_eq!(sign_in_response.status_code(), 303);
    }

    #[tokio::test]
    async fn sign_in_invalid() {
        let state = Arc::new(Mutex::new(Server::new()));
        let app = build_app_with_state(state.clone());
        let server = TestServer::new(app).unwrap();

        let sign_up_response = server
            .post("/sign_up")
            .json(&json!({
                "email": "1@email.com",
                "password": "12345678",
                "name": "Juniper",
            }))
            .await;

        sign_up_response.assert_status_ok();

        // Account doesn't exist
        let bad_email = server
            .post("/sign_in")
            .json(&json!({
                "email": "2@email.com",
                "password": "12345678"
            }))
            .await;

        bad_email.assert_status_not_ok();

        // Password is wrong
        let bad_password = server
            .post("/sign_in")
            .json(&json!({
                "email": "1@email.com",
                "password": "abcdefgh"
            }))
            .await;

        bad_password.assert_status_not_ok();
    }

    #[tokio::test]
    async fn sign_out_simple() {
        let state = Arc::new(Mutex::new(Server::new()));
        let app = build_app_with_state(state.clone());
        let mut server = TestServer::new(app).unwrap();
        server.save_cookies();

        let _sign_up_response = server
            .post("/sign_up")
            .json(&json!({
                "email": "1@email.com",
                "password": "12345678",
                "name": "Juniper",
            }))
            .await;

        let sign_in_response = server
            .post("/sign_in")
            .json(&json!({
                "email": "1@email.com",
                "password": "12345678"
            }))
            .await;

        let location = sign_in_response.headers().get("Location").unwrap();
        assert_eq!(location, "/dashboard");

        let sign_out_response = server.post("/sign_out").await;

        let location = sign_out_response.headers().get("Location").unwrap();
        assert_eq!(location, "/");
    }
}
