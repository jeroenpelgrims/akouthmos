mod answer;
mod feedback;
mod languages;
mod settings;

use askama::Template;
use askama_web::WebTemplate;

pub use answer::{AnswerTemplate, answer};
pub use feedback::{FeedbackTemplate, feedback, word};
pub use settings::SettingsTemplate;

pub struct GameSettings {
    pub min: u64,
    pub max: u64,
    pub language: &'static str,
    pub speed: f64,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            min: 0,
            max: 100000,
            language: "el",
            speed: 1.0,
        }
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "router/index/index.html")]
pub struct IndexTemplate {
    pub number: u64,
    pub settings: SettingsTemplate,
    pub answer: AnswerTemplate,
    pub feedback: FeedbackTemplate,
}

pub async fn index() -> IndexTemplate {
    let settings = GameSettings::default();
    let number = rand::random_range(settings.min..=settings.max);
    IndexTemplate {
        number,
        settings: SettingsTemplate::from_game_settings(settings),
        answer: AnswerTemplate { number },
        feedback: FeedbackTemplate::default(),
    }
}
