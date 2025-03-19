#![doc = include_str!("../readme.md")]

pub use anyhow::{Context as _, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
/// Quote structure
pub struct Quote {
    /// quote text
    #[serde(rename(deserialize = "quoteText", serialize = "text"))]
    pub quote_text: String,
    /// quote author, may be absent
    #[serde(rename(deserialize = "quoteAuthor", serialize = "author"))]
    pub quote_author: String,
}

impl Quote {
    /// Serializes a [`Quote`] to a JSON string.
    ///
    /// # Errors
    ///
    /// Returns an error on parsing failure.
    pub fn serialize(self) -> Result<String> {
        serde_json::to_string(&self).context("failed to serialize Quote")
    }

    /// Deserializes a JSON representation of a [`Quote`].
    ///
    /// This function correctly handles inaccurately escaped apostrophes, which occur regularly in API responses from Forismatic.
    ///
    /// # Errors
    ///
    /// Returns an error on parsing failure.
    pub fn deserialize(response: &str) -> Result<Self> {
        let fixed_response = response.replace("\\'", "'"); // i really hate this API

        serde_json::from_str::<Self>(&fixed_response).context("failed to deserialize JSON")
    }
}

/// Gets a quote from the API and deserializes it to a [`Quote`].
///
/// # Errors
///
/// Returns an error in several cases:
///
/// * GET request fails
/// * body cannot by parsed to a UTF-8 string
/// * fails at deserializing [`String`] to [`Quote`]
pub fn get_quote(lang: &str) -> Result<Quote> {
    let uri =
        format!("https://api.forismatic.com/api/1.0/?method=getQuote&format=json&lang={lang}");

    let response = request(&uri)?;

    Quote::deserialize(&response)
}

/// Performs a GET request using [`ureq`] and returns the body as a [`String`].
///
/// # Errors
///
/// Returns an error if the GET request fails or if the body cannot be parsed to a UTF-8 string.
fn request(uri: &str) -> Result<String> {
    let mut response = ureq::get(uri).call().context("request error")?;
    let string = response
        .body_mut()
        .read_to_string()
        .context("failed to read response")?;

    Ok(string)
}
