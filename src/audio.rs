use std::sync::{Arc, LazyLock};

use moka::future::Cache;
use urlencoding::encode;

const AUDIO_CACHE_MAX_BYTES: u64 = 50 * 1024 * 1024;

static AUDIO_CACHE: LazyLock<Cache<(String, String), Vec<u8>>> = LazyLock::new(|| {
    Cache::builder()
        .max_capacity(AUDIO_CACHE_MAX_BYTES)
        .weigher(|key: &(String, String), value: &Vec<u8>| {
            (key.0.len() + key.1.len() + value.len())
                .try_into()
                .unwrap_or(u32::MAX)
        })
        .build()
});

pub fn get_url(text: &str, language_code: &str) -> String {
    format!(
        "https://translate.google.com/translate_tts?ie=UTF-8&tl={}&client=tw-ob&q={}",
        language_code,
        encode(text)
    )
}

async fn fetch_audio(url: &str) -> Result<Vec<u8>, anyhow::Error> {
    tracing::info!(%url, "Fetching audio");
    let response = reqwest::get(url).await?.error_for_status()?;
    let bytes = response.bytes().await?;
    Ok(bytes.to_vec())
}

pub async fn get_audio(text: &str, language_code: &str) -> Result<Vec<u8>, anyhow::Error> {
    let key = (language_code.to_owned(), text.to_owned());
    let url = get_url(text, language_code);

    AUDIO_CACHE
        .try_get_with(key, async move { fetch_audio(&url).await })
        .await
        .map_err(|error: Arc<anyhow::Error>| anyhow::anyhow!(error.to_string()))
}
