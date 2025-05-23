#![doc = include_str!("../readme.md")]

pub mod cli;
pub mod quote;

use anyhow::Error;
use cli::Args;
use owo_colors::OwoColorize as _;
use quote::Backend;
use quote::Quote;

pub const GUILLEMETS: (&str, &str) = ("«", "»");
pub const CURLY_QUOTES: (&str, &str) = ("“", "”");
pub const GERMAN_QUOTES: (&str, &str) = ("„", "“");

pub const FORISMATIC_URL: &str = "https://api.forismatic.com/api/1.0/?method=getQuote&format=json";
pub const HAPESIRE_URL: &str = "https://hapesire.kira.computer/api/quotes/random";

/// Prints out the quote and it's author (if not absent) using [`Args`].
pub fn get_quote_and_print(args: &Args) -> Result<(), Error> {
    let language: &str = args.get_language();

    let quote_struct: Quote = match args.backend.to_lowercase().as_str() {
        "forismatic" => Backend::Forismatic { language }.get_quote_and_parse()?,
        "hapesire" | _ => Backend::Hapesire { language }.get_quote_and_parse()?,
    };

    if args.json {
        let json: String = quote_struct.serialize_to_json()?;

        println!("{json}");
    } else {
        let Quote { mut text, author } = quote_struct;

        textwrap::fill_inplace(&mut text, args.wrap_width - 2);

        let (text_style, author_style) = args.get_colors();
        let (left_quote, right_quote) = args.get_quotation_marks(language);

        text = replace_quotations(&text, language);

        println!("{left_quote}{}{right_quote}", text.style(text_style));

        if let Some(author) = author {
            println!("{}", author.style(author_style));
        }
    }

    Ok(())
}

/// Returns new [`String`] with quotation marks replaced according to the language.
fn replace_quotations(text: &str, language: &str) -> String {
    if language == "ru" {
        text.replace(GUILLEMETS.0, GERMAN_QUOTES.0)
            .replace(GUILLEMETS.1, GERMAN_QUOTES.1)
    } else {
        text.replace('"', "'")
    }
}
