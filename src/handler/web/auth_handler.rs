use askama::Template;
use axum::{Form, response::IntoResponse};
use serde::Deserialize;

#[derive(Template)]
#[template(path = "pages/auth/login.html")]
pub struct LoginTemplate;

#[derive(Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

pub async fn login_handler() -> impl IntoResponse {
    let base = LoginTemplate;
    return axum::response::Html::from(base.render().unwrap());
}

pub async fn login_form_handler(Form(login_form): Form<LoginForm>) {
    println!("Pidor trying to login by {} using {}", login_form.username, login_form.password);
}