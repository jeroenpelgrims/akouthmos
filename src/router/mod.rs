use axum::{
    Router,
    routing::{get, post},
};
use tower_http::services::ServeDir;

mod audio;
mod index;

pub fn router() -> Router {
    Router::new()
        .route("/", get(index::index))
        .route("/answer", get(index::answer))
        .route("/feedback", post(index::feedback))
        .route("/word", get(index::word))
        .route("/audio/{language}/{text}", get(audio::audio))
        .route("/audio/preview", get(audio::audio_preview))
        .nest_service("/static", ServeDir::new("static"))
}
