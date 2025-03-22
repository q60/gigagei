use anyhow::Error;
use argh::FromArgs;
use gigagei::{Backend, Quote};
use owo_colors::{OwoColorize as _, Style};

const GUILLEMETS: (&str, &str) = ("«", "»");
const CURLY_QUOTES: (&str, &str) = ("“", "”");
const GERMAN_QUOTES: (&str, &str) = ("„", "“");

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

    /// return quote in JSON
    #[argh(switch, short = 'j')]
    json: bool,

    /// wrap width in characters, default is terminal width
    #[argh(option, short = 'w', default = "textwrap::termwidth()")]
    wrap_width: usize,
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
            ("en" | "ru", true) => ("\"", "\""),
            ("ru", false) => GUILLEMETS,
            _ => CURLY_QUOTES,
        }
    }
}

fn main() -> Result<(), Error> {
    let args: Args = argh::from_env();

    let language: &str = args.get_language();

    let mut quote_struct: Quote = Backend::Forismatic { language }.get_quote_and_parse()?;

    quote_struct.text = quote_struct.text.trim().to_string();
    quote_struct.author = quote_struct.author.trim().to_string();

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

        if !author.is_empty() {
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
