use std::sync::LazyLock;

use anyhow::Result;
use moka::future::Cache;
use rig::{agent::AgentBuilder, providers::openai::OpenAIConfig};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

type NumberKey = (u64, String);

static NUMBER_CACHE: LazyLock<Cache<NumberKey, NumberWord>> = LazyLock::new(|| Cache::new(100_000));

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct NumberWord {
    pub word: String,
}

fn agent(preamble: &str) -> AgentBuilder {
    let api_key = std::env::var("OPENROUTER_API_KEY")
        .expect("Failed to load OPENROUTER_API_KEY from environment");
    let model_name = std::env::var("OPENROUTER_MODEL")
        .expect("Failed to load OPENROUTER_MODEL from environment");
    let model = OpenAIConfig::new(api_key)
        .with_base_url("https://openrouter.ai/api/v1")
        .client()
        .chat(model_name);

    AgentBuilder::new(model).preamble(preamble)
}

pub async fn number_to_word(number: u64, language_code: &str) -> Result<NumberWord> {
    let key = (number, language_code.to_owned());
    if let Some(result) = NUMBER_CACHE.get(&key).await {
        return Ok(result);
    }

    tracing::info!(number, language_code, "Converting number to word with LLM");
    let agent = agent(
        "You are an expert linguist.
        Write out the given number as a whole, in words, in the language identified by the given ISO 639-1 language code.
        The result must be the natural way a native speaker pronounces the entire number when reading it aloud
        (e.g. 42 in English is \"forty-two\", NOT \"four two\"; 1234 is \"one thousand two hundred thirty-four\", NOT \"one two three four\").
        Return only the number written out in words, without any digits, annotations, or other text.",
    )
    .build();
    let response = agent
        .prompt_typed::<NumberWord>(format!(
            "Write out the number {number} in words in the language with code '{language_code}'."
        ))
        .await?;
    let result = response.output;
    NUMBER_CACHE.insert(key, result.clone()).await;

    Ok(result)
}
