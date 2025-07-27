mod app;
mod auth;
mod gameplay;
mod models;
mod routes;
mod tests;

use crate::app::build_app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = build_app();

    // Run our app with hyper, listening globally on port 3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    println!("Backend running at http://localhost:3000/");

    Ok(())
}
