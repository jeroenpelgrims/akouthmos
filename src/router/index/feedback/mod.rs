use askama::Template;
use askama_web::WebTemplate;

#[derive(Template, WebTemplate)]
#[template(path = "router/index/feedback/feedback.html")]
pub struct FeedbackTemplate {
    pub correct: bool,
    pub number: String,
    pub guess: String,
    pub language_name: String,
}

impl Default for FeedbackTemplate {
    fn default() -> Self {
        Self {
            correct: true,
            number: "42".into(),
            guess: String::new(),
            language_name: "Greek".into(),
        }
    }
}
