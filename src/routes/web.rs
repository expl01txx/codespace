use std::sync::Arc;

use axum::{Router, routing::{get, post}};

use crate::{app_state::AppState, handler};

pub fn routes() -> Router<Arc<AppState>>  {
    Router::new()
        .route("/", get(handler::web::base_handler::index_handler))
        .route("/auth/login", get(handler::web::auth_handler::login_handler))
        .route("/auth/login", post(handler::web::auth_handler::login_form_handler))
}