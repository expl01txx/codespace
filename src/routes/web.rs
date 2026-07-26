use std::sync::Arc;

use axum::{Router, routing::{get, post}};

use crate::{app_state::AppState, handler};

pub fn routes() -> Router<Arc<AppState>>  {
    Router::new()
        .route("/", get(handler::web::base_handler::index_handler))
        .route("/auth/login", get(handler::web::auth_handler::login_page))
        .route("/auth/login", post(handler::web::auth_handler::login_handler))
        .route("/auth/register", get(handler::web::auth_handler::register_page))
        .route("/auth/register", post(handler::web::auth_handler::register_handler))
}