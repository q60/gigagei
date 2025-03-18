use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Quote {
    quote_text: String,
    quote_author: String,
}

const YELLOW: &str = "\x1B[93m";
const BLUE: &str = "\x1B[94m";
const BOLD: &str = "\x1B[1m";
const RESET: &str = "\x1B[0m";

fn main() -> Result<()> {
    let uri = "https://api.forismatic.com/api/1.0/?method=getQuote&format=json&lang=en";

    let response = get_request(uri)?;
    let quote = parse(&response)?;

    let text = textwrap::fill(quote.quote_text.trim(), 60);
    let author = quote.quote_author.trim();

    println!("\"{BLUE}{BOLD}{text}{RESET}\"");

    if !author.is_empty() {
        println!("{YELLOW}{author}{RESET}");
    }

    Ok(())
}

fn parse(response: &str) -> Result<Quote> {
    let fixed_response = response.replace("\\'", "'"); // i really hate this API

    serde_json::from_str::<Quote>(&fixed_response).context("failed to serialize JSON")
}

fn get_request(uri: &str) -> Result<String> {
    let mut response = ureq::get(uri).call().context("request error")?;
    let string = response
        .body_mut()
        .read_to_string()
        .context("failed to read response")?;

    Ok(string)
}
