use axum::{Router, routing::get};
use tower_http::services::ServeDir;

mod audio;
mod index;

pub fn router() -> Router {
    Router::new()
        .route("/", get(index::index))
        .route("/audio/{language}/{text}", get(audio::audio))
        .route("/audio/preview", get(audio::audio_preview))
        .nest_service("/static", ServeDir::new("static"))
}
