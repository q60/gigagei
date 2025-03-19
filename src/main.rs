#![doc = include_str!("../readme.md")]

use anyhow::{Context as _, Result};
use argh::FromArgs;
use owo_colors::{OwoColorize as _, Style};
use serde::Deserialize;

#[derive(FromArgs)]
#[argh(help_triggers("-h", "--help", "help"))]
/// A random quote fetching console utility
struct Args {
    /// quote language, must be one of: en\[glish\], ru\[ssian\]
    #[argh(option, short = 'l', default = "\"en\".to_string()")]
    language: String,

    /// force ASCII quotation marks
    #[argh(switch, short = 'a')]
    ascii_quotation: bool,

    /// disables colors
    #[argh(switch, short = 'n')]
    no_colors: bool,
}

impl Args {
    /// Returns a tuple of styles to use on quote and its author.
    fn get_colors(&self) -> (Style, Style) {
        if self.no_colors {
            let s = Style::new();
            (s, s)
        } else {
            (
                Style::new().bright_blue().bold(),
                Style::new().bright_yellow(),
            )
        }
    }

    /// Returns a language code from the `ascii_quotation` option.
    fn get_language(&self) -> &str {
        if self.language.to_lowercase().starts_with("en") {
            "en"
        } else {
            "ru"
        }
    }

    /// Returns a tuple of quotation marks to use on quotes, considers `ascii_quotation` option and language.
    fn get_quotation_marks(&self, lang: &str) -> (&str, &str) {
        match (lang, self.ascii_quotation) {
            ("en", true) | ("ru", true) => ("\"", "\""),
            ("ru", false) => ("«", "»"),
            _ => ("“", "”"),
        }
    }
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
    let args: Args = argh::from_env();

    let lang = args.get_language();
    let (text_style, author_style) = args.get_colors();
    let (left_quote, right_quote) = args.get_quotation_marks(lang);

    let uri =
        format!("https://api.forismatic.com/api/1.0/?method=getQuote&format=json&lang={lang}");

    let response = get_request(&uri)?;
    let quote = parse(&response)?;

    let text = textwrap::fill(quote.quote_text.trim(), 60);
    let author = quote.quote_author.trim();

    let text = if lang == "en" {
        text.replace("\"", "'")
    } else {
        text
    };

    println!("{left_quote}{}{right_quote}", text.style(text_style));

    if !author.is_empty() {
        println!("{}", author.style(author_style));
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
