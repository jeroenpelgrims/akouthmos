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

#[derive(Deserialize)]
pub struct AudioPreviewParams {
    pub lang: String,
    pub text: String,
    pub speed: Option<f64>,
}

pub async fn audio_preview(Query(params): Query<AudioPreviewParams>) -> String {
    let speed = params.speed.unwrap_or(1.0).clamp(0.25, 4.0);
    format!(
        "<audio autoplay onloadedmetadata=\"this.playbackRate = {speed};\" src=\"/audio/{}/{}\"></audio>",
        params.lang, params.text
    )
}
