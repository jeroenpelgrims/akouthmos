mod answer;
mod feedback;
mod languages;
mod settings;

use askama::Template;
use askama_web::WebTemplate;

pub use answer::AnswerTemplate;
pub use feedback::FeedbackTemplate;
pub use settings::SettingsTemplate;

#[derive(Template, WebTemplate)]
#[template(path = "router/index/index.html")]
pub struct IndexTemplate {
    pub number: u64,
    pub settings: String,
    pub answer: String,
    pub feedback: String,
}

pub async fn index() -> IndexTemplate {
    let settings = SettingsTemplate::default();
    let number = rand::random_range(settings.min..=settings.max);
    IndexTemplate {
        number,
        settings: settings.render().unwrap_or_default(),
        answer: AnswerTemplate.render().unwrap_or_default(),
        feedback: FeedbackTemplate::default().render().unwrap_or_default(),
    }
}
