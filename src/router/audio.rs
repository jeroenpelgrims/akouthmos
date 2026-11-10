use askama::Template;
use askama_web::WebTemplate;
use axum::{
    extract::{Path, Query},
    http::{StatusCode, header},
    response::IntoResponse,
};
use serde::Deserialize;

use crate::audio;

pub async fn audio(Path((language, text)): Path<(String, String)>) -> impl IntoResponse {
    match audio::get_audio(&text, &language).await {
        Ok(audio_data) => {
            ([(header::CONTENT_TYPE, "audio/octet-stream")], audio_data).into_response()
        }
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "router/audio_preview.html")]
pub struct AudioPreviewTemplate {
    pub lang: String,
    pub text: String,
    pub speed: f64,
}

#[derive(Deserialize)]
pub struct AudioPreviewParams {
    pub lang: String,
    pub text: String,
    pub speed: Option<f64>,
}

impl From<AudioPreviewParams> for AudioPreviewTemplate {
    fn from(params: AudioPreviewParams) -> Self {
        AudioPreviewTemplate {
            lang: params.lang,
            text: params.text,
            speed: params.speed.unwrap_or(1.0).clamp(0.25, 4.0),
        }
    }
}

pub async fn audio_preview(Query(params): Query<AudioPreviewParams>) -> AudioPreviewTemplate {
    params.into()
}
