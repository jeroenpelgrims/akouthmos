use askama::Template;
use askama_web::WebTemplate;
use axum::extract::Query;
use serde::Deserialize;

use crate::router::index::GameSettings;

#[derive(Template, WebTemplate)]
#[template(path = "router/index/answer/answer.html")]
pub struct AnswerTemplate {
    pub number: u64,
}

#[derive(Deserialize)]
pub struct AnswerParams {
    min: Option<String>,
    max: Option<String>,
}

pub async fn answer(Query(params): Query<AnswerParams>) -> AnswerTemplate {
    let defaults = GameSettings::default();
    let min = params
        .min
        .and_then(|v| v.parse().ok())
        .unwrap_or(defaults.min);
    let max = params
        .max
        .and_then(|v| v.parse().ok())
        .unwrap_or(defaults.max);
    let (min, max) = if min <= max { (min, max) } else { (max, min) };
    AnswerTemplate {
        number: rand::random_range(min..=max),
    }
}
