use askama::Template;
use askama_web::WebTemplate;

use super::languages::{LANGUAGES, Language};

pub static SPEEDS: &[(f64, &str)] = &[(0.5, "Slow"), (0.75, "Medium"), (1.0, "Normal")];

#[derive(Template, WebTemplate)]
#[template(path = "router/index/settings/settings.html")]
pub struct SettingsTemplate {
    pub languages: &'static [Language],
    pub default_language: &'static str,
    pub speeds: &'static [(f64, &'static str)],
    pub default_speed: f64,
    pub min: u64,
    pub max: u64,
}

impl Default for SettingsTemplate {
    fn default() -> Self {
        Self {
            languages: LANGUAGES,
            default_language: "el",
            speeds: SPEEDS,
            default_speed: 1.0,
            min: 0,
            max: 10000000000,
        }
    }
}
