//! Quotes API module.

use super::FORISMATIC_URL;
use super::HAPESIRE_URL;
use anyhow::Context as _;
use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;

/// Enumerates quotes backends.
pub enum Backend<'a> {
    /// Forismatic backend for quotes, provides quotes in English and Russian.
    Forismatic { language: &'a str },

    /// Forismatic backend for quotes, provides quotes in English and Russian.
    Hapesire { language: &'a str },
}

impl Backend<'_> {
    /// Gets a quote from the API and deserializes it to a [`Quote`].
    ///
    /// This function replaces inaccurately escaped apostrophes, which occur regularly in API responses from Forismatic.
    ///
    /// # Errors
    ///
    /// Returns an error in several cases:
    ///
    /// * GET request fails
    /// * body cannot be parsed to a UTF-8 string
    /// * fails at deserializing [`String`] to [`Quote`]
    pub fn get_quote_and_parse(&self) -> Result<Quote> {
        match self {
            Self::Forismatic { language } => Forismatic::get_quote_and_parse(language),
            Self::Hapesire { language } => Hapesire::get_quote_and_parse(language),
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
/// Forismatic API response structure.
struct Forismatic {
    /// quote text.
    quote_text: String,

    /// quote author, may be absent.
    quote_author: String,
}

#[derive(Deserialize)]
struct Hapesire {
    data: HapesireObject,
}

#[derive(Deserialize)]
struct HapesireObject {
    attributes: Quote,
}

impl Forismatic {
    /// Gets a quote from the API and deserializes it to a [`Quote`].
    ///
    /// This function replaces inaccurately escaped apostrophes, which occur regularly in API responses from Forismatic.
    ///
    /// # Errors
    ///
    /// Returns an error in several cases:
    ///
    /// * GET request fails
    /// * body cannot be parsed to a UTF-8 string
    /// * fails at deserializing [`String`] to [`Quote`]
    pub fn get_quote_and_parse(lang: &str) -> Result<Quote> {
        let uri = format!("{FORISMATIC_URL}&lang={lang}");

        let response = request_get(&uri)?.replace("\\'", "'"); // i really hate this API

        let Self {
            quote_text,
            quote_author,
        } = serde_json::from_str::<Self>(&response).context("failed to deserialize JSON")?;

        let text = quote_text.trim().to_string();
        let author = quote_author.trim().to_string();

        Ok(Quote {
            text,
            author: if author.is_empty() {
                None
            } else {
                Some(author)
            },
        })
    }
}

impl Hapesire {
    /// Gets a quote from the API and deserializes it to a [`Quote`].
    ///
    /// This function replaces inaccurately escaped apostrophes, which occur regularly in API responses from Forismatic.
    ///
    /// # Errors
    ///
    /// Returns an error in several cases:
    ///
    /// * GET request fails
    /// * body cannot be parsed to a UTF-8 string
    /// * fails at deserializing [`String`] to [`Quote`]
    pub fn get_quote_and_parse(lang: &str) -> Result<Quote> {
        let uri = format!("{HAPESIRE_URL}/{lang}");

        let response = request_get(&uri)?;
        let object =
            serde_json::from_str::<Self>(&response).context("failed to deserialize JSON")?;

        Ok(object.data.attributes)
    }
}

#[derive(Serialize, Deserialize, Clone)]
/// Quote structure.
pub struct Quote {
    /// quote text.
    pub text: String,

    /// quote author.
    pub author: Option<String>,
}

impl Quote {
    /// Serializes a [`Quote`] to a JSON string.
    ///
    /// # Errors
    ///
    /// Returns an error on parsing failure.
    pub fn serialize_to_json(self) -> Result<String> {
        serde_json::to_string(&self).context("failed to serialize Quote")
    }
}

/// Performs a GET request using [`ureq`] and returns the body as a [`String`].
///
/// # Errors
///
/// Returns an error if the GET request fails or if the body cannot be parsed to a UTF-8 string.
pub fn request_get(uri: &str) -> Result<String> {
    let mut response = ureq::get(uri).call().context("request error")?;
    let string = response
        .body_mut()
        .read_to_string()
        .context("failed to read response")?;

    Ok(string)
}
