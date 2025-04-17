//! CLI module.

use super::CURLY_QUOTES;
use super::GUILLEMETS;
use argh::FromArgs;
use owo_colors::Style;

#[derive(FromArgs)]
#[argh(help_triggers("-h", "--help", "help"))]
/// A random quote fetching console utility
pub struct Args {
    /// quote language, must be one of: en\[glish\], ru\[ssian\]
    #[argh(option, short = 'l', default = "\"en\".to_string()")]
    pub language: String,

    /// force ASCII quotation marks
    #[argh(switch, short = 'a')]
    pub ascii_quotation: bool,

    /// disables colors
    #[argh(switch, short = 'n')]
    pub no_colors: bool,

    /// return quote in JSON
    #[argh(switch, short = 'j')]
    pub json: bool,

    /// wrap width in characters, default is terminal width
    #[argh(option, short = 'w', default = "textwrap::termwidth()")]
    pub wrap_width: usize,
}

impl Args {
    /// Returns a tuple of styles to use on quote and its author.
    pub fn get_colors(&self) -> (Style, Style) {
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
    pub fn get_language(&self) -> &str {
        if self.language.to_lowercase().starts_with("en") {
            "en"
        } else {
            "ru"
        }
    }

    /// Returns a tuple of quotation marks to use on quotes, considers `ascii_quotation` option and language.
    pub fn get_quotation_marks(&self, lang: &str) -> (&str, &str) {
        match (lang, self.ascii_quotation) {
            ("en" | "ru", true) => ("\"", "\""),
            ("ru", false) => GUILLEMETS,
            _ => CURLY_QUOTES,
        }
    }
}
