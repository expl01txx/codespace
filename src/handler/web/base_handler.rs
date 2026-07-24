use askama::Template;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "pages/index.html")]
pub struct BaseTemplate;

pub async fn index_handler() -> impl IntoResponse {
    let base = BaseTemplate;
    return axum::response::Html::from(base.render().unwrap());
}