use ureq::{get, Error};
use xmlparser::{Token, Tokenizer};

fn main() -> Result<(), Error> {
    let uri: String = ["https://api.forismatic.com/api/1.0/?",
                       "method=getQuote&format=xml&lang=en"].join("");
    match get(&uri).call() {
        Ok(response) => {
            let text = response.into_string()?;
            process_response(text);
            Ok(())
        }
        Err(Error::Status(code, _response)) => {
            println!("\x1B[94m\x1B[1mHTTP error:\x1B[0m \x1B[91m{}", code);
            Ok(()) // no error if http error
        }
        Err(e) => {
            println!("\x1B[91m\x1B[1mUnknown error.");
            Err(e) // error if something bad happened...
        }
    }
}

fn process_response(text: String) {
    let data = Tokenizer::from(text.as_str())
        .fold(Vec::new(), |mut res, t| {
            if let Ok(token) = t {
                if let Token::Text { text } = token {
                    res.push(text.as_str().trim());
                }
            }
            res
        });
    println!(
        "\"\x1B[94m\x1B[1m{}\x1B[0m\"",
        textwrap::fill(data[0], 60).replace("\n", "\n "),
    );
    if data.len() == 3 {
        println!("\x1B[93m{}\x1B[0m", data[1]);
    }
}
