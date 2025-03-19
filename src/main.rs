#![doc = include_str!("../readme.md")]

use anyhow::{Context as _, Result};
use argh::FromArgs;
use owo_colors::OwoColorize as _;
use serde::Deserialize;

#[derive(FromArgs)]
/// A random quote fetching console utility
struct Args {
    /// quote language, must be one of: en\[glish\], ru\[ssian\]
    #[argh(option, short = 'l')]
    language: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
/// Quote structure
struct Quote {
    /// Quote text
    quote_text: String,
    /// Quote author, may be absent
    quote_author: String,
}

fn main() -> Result<()> {
    let uri = prepare_uri();

    let response = get_request(&uri)?;
    let quote = parse(&response)?;

    let text = textwrap::fill(quote.quote_text.trim(), 60);
    let author = quote.quote_author.trim();

    println!("\"{}\"", text.bright_blue().bold());

    if !author.is_empty() {
        println!("{}", author.bright_yellow());
    }

    Ok(())
}

/// Performs a GET request using [`ureq`] and returns the body as a [String].
///
/// # Errors
///
/// Returns an error if the GET request fails or if the body cannot be parsed to a UTF-8 string.
fn get_request(uri: &str) -> Result<String> {
    let mut response = ureq::get(uri).call().context("request error")?;
    let string = response
        .body_mut()
        .read_to_string()
        .context("failed to read response")?;

    Ok(string)
}

/// Deserializes a JSON representation of a [`Quote`].
///
/// This function correctly handles inaccurately escaped apostrophes, which occur regularly in API responses from Forismatic.
///
/// # Errors
///
/// Returns an error on parsing failure.
fn parse(response: &str) -> Result<Quote> {
    let fixed_response = response.replace("\\'", "'"); // i really hate this API

    serde_json::from_str::<Quote>(&fixed_response).context("failed to deserialize JSON")
}

/// Returns API endpoint as a [`String`].
///
/// This function checks for command line arguments and uses supplied language option if specified. Constructs URI according to this option.
fn prepare_uri() -> String {
    let Args { language } = argh::from_env();

    let lang = if language
        .unwrap_or_default()
        .to_lowercase()
        .starts_with("ru")
    {
        "ru"
    } else {
        "en"
    };

    format!("https://api.forismatic.com/api/1.0/?method=getQuote&format=json&lang={lang}")
}
