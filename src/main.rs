use std::sync::Arc;

use axum::Router;
use migration::{Migrator, MigratorTrait}; 
use crate::{app_state::AppState, services::auth_service::{self, AuthService}};

mod db;
mod models;
mod handler;
mod routes;
mod app_state;
mod services;
mod errors;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db = db::connect_to_db().await.unwrap();
    
    Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations");
    
    println!("[✓] Database migrations applied");

    let state = AppState { db };
    let state = Arc::new(state);

    let app = Router::<Arc<AppState>>::new()
        .merge(routes::web::routes())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("[✓] Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}