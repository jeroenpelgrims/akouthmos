use askama::Template;
use askama_web::WebTemplate;

use crate::router::index::GameSettings;

use super::languages::{LANGUAGES, Language};

pub static SPEEDS: &[(f64, &str)] = &[(0.5, "Slow"), (0.75, "Medium"), (1.0, "Normal")];

#[derive(Template, WebTemplate)]
#[template(path = "router/index/settings/settings.html")]
pub struct SettingsTemplate {
    pub languages: &'static [Language],
    pub selected_language: &'static str,
    pub speeds: &'static [(f64, &'static str)],
    pub selected_speed: f64,
    pub min: u64,
    pub max: u64,
}

impl SettingsTemplate {
    pub fn from_game_settings(settings: GameSettings) -> Self {
        SettingsTemplate {
            languages: LANGUAGES,
            selected_language: settings.language,
            speeds: SPEEDS,
            selected_speed: settings.speed,
            min: settings.min,
            max: settings.max,
        }
    }
}
