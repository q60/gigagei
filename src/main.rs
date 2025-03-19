use argh::FromArgs;
use gigagei::{Quote, Result};
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

fn main() -> Result<()> {
    let args: Args = argh::from_env();

    let lang: &str = args.get_language();

    let mut quote_struct: Quote = gigagei::get_quote(lang)?;

    quote_struct.quote_text = quote_struct.quote_text.trim().to_string();
    quote_struct.quote_author = quote_struct.quote_author.trim().to_string();

    if args.json {
        let json: String = quote_struct.serialize()?;

        println!("{json}");
    } else {
        let Quote {
            mut quote_text,
            quote_author,
        } = quote_struct;

        textwrap::fill_inplace(&mut quote_text, args.wrap_width - 2);

        let (text_style, author_style) = args.get_colors();
        let (left_quote, right_quote) = args.get_quotation_marks(lang);

        quote_text = replace_quotations(&quote_text, lang);

        println!("{left_quote}{}{right_quote}", quote_text.style(text_style));

        if !quote_author.is_empty() {
            println!("{}", quote_author.style(author_style));
        }
    }

    Ok(())
}

/// Returns new [`String`] with quotation marks replaced according to the language.
fn replace_quotations(text: &str, lang: &str) -> String {
    if lang == "ru" {
        text.replace(GUILLEMETS.0, GERMAN_QUOTES.0)
            .replace(GUILLEMETS.1, GERMAN_QUOTES.1)
    } else {
        text.replace('"', "'")
    }
}
