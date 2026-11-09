use askama::Template;
use askama_web::WebTemplate;

#[derive(Template, WebTemplate)]
#[template(path = "router/index/answer/answer.html")]
pub struct AnswerTemplate;
