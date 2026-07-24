use std::sync::Arc;

use axum::Router;

use crate::app_state::AppState;

mod db;
mod models;
mod handler;
mod routes;
mod app_state;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db = db::connect_to_db().await.unwrap();

    let state = AppState {
        db,
    };

    let state = Arc::new(state);

    let app = Router::<Arc<AppState>>::new()
        .merge(routes::web::routes())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}