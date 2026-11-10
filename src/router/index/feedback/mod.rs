use askama::Template;
use askama_web::WebTemplate;
use axum::{
    Form,
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Deserialize;

use super::GameSettings;
use crate::llm;

#[derive(Template, WebTemplate)]
#[template(path = "router/index/feedback/feedback.html")]
pub struct FeedbackTemplate {
    pub correct: bool,
    pub number: String,
    pub guess: String,
}

impl Default for FeedbackTemplate {
    fn default() -> Self {
        Self {
            correct: true,
            number: "42".into(),
            guess: String::new(),
        }
    }
}

#[derive(Deserialize)]
pub struct FeedbackForm {
    number: u64,
    guess: u64,
}

pub async fn feedback(Form(form): Form<FeedbackForm>) -> FeedbackTemplate {
    FeedbackTemplate {
        correct: form.guess == form.number,
        number: form.number.to_string(),
        guess: form.guess.to_string(),
    }
}

#[derive(Deserialize)]
pub struct WordParams {
    number: u64,
    lang: Option<String>,
}

pub async fn word(Query(params): Query<WordParams>) -> Response {
    let default_language = GameSettings::default().language;
    let lang = params.lang.as_deref().unwrap_or(default_language);
    match llm::number_to_word(params.number, lang).await {
        Ok(word) => word.word.into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}
