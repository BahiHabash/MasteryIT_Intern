mod errors;
mod handlers;
mod models;
mod state;

use anyhow::Context;
use axum::{
    routing::{delete, get},
    Router,
};
use state::{AppState, SharedState};
use std::sync::{Arc, Mutex};
use tower_http::catch_panic::CatchPanicLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let shared_state: SharedState = Arc::new(Mutex::new(AppState::new()));

    let app = Router::new()
        .route(
            "/books",
            get(handlers::list_books).post(handlers::create_book),
        )
        .route("/books/:id", delete(handlers::delete_book_handler))
        .route("/panic", get(handlers::trigger_panic))
        .with_state(shared_state)
        .layer(CatchPanicLayer::new());

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .with_context(|| format!("Failed to bind TCP listener to address {}", addr))
        .expect("Critical startup failure");

    println!("Server running on http://{}", addr);
    println!("Available endpoints:");
    println!("  GET    /books");
    println!("  POST   /books");
    println!("  DELETE /books/:id");
    println!("  GET    /panic  (Simulates a thread panic)");

    axum::serve(listener, app)
        .await
        .context("Axum server encountered a fatal error during execution")?;

    Ok(())
}
