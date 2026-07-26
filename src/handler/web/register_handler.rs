use std::sync::Arc;
use askama::Template;
use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
    Form,
};
use serde::Deserialize;
use crate::app_state::AppState;
use crate::services::auth_service::AuthInput;
use crate::AuthService;

#[derive(Template)]
#[template(path = "pages/auth/register.html")]
pub struct RegisterTemplate {
    pub error: Option<String>,
    pub username: Option<String>,
}

#[derive(Deserialize)]
pub struct RegisterForm {
    pub username: String,
    pub password: String,
}

pub async fn register_page() -> impl IntoResponse {
    let template = RegisterTemplate {
        error: None,
        username: None,
    };
    axum::response::Html(template.render().unwrap())
}

pub async fn register_handler(
    State(state): State<Arc<AppState>>,
    Form(form): Form<RegisterForm>,
) -> impl IntoResponse {
    let input = AuthInput {
        username: form.username.clone(),
        password: form.password,
    };

    let auth_service = AuthService::new(state.db.clone());
    match auth_service.register(input).await {
        Ok(_) => Redirect::to("/auth/login").into_response(),
        Err(e) => {
            let template = RegisterTemplate {
                error: Some(e.to_string()),
                username: Some(form.username),
            };
            axum::response::Html(template.render().unwrap()).into_response()
        }
    }
}