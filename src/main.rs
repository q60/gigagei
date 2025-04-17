use anyhow::Error;
use gigagei::cli::Args;
use gigagei::get_quote_and_print;

fn main() -> Result<(), Error> {
    let args: Args = argh::from_env();

    get_quote_and_print(&args)
}
