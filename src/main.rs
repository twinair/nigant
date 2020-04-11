#![deny(warnings)]
extern crate reqwest;

use structopt::StructOpt;
use anyhow::Result;

mod parser;
mod request;
mod output;
mod cache;


#[derive(StructOpt, Debug)]
#[structopt(name = "nigant", about = "A thesaurus for the terminal")]
pub struct Cli {
    /// Word to Look-up
    word: String,

    /// Fetch Synonyms for the Word
    #[structopt(short, long)]
    synonyms: bool,

    /// Fetch Etymology of the Word
    #[structopt(short, long)]
    etymology: bool,


    /// Activate Debug Mode
    #[structopt(short, long)]
    debug: bool,

    /// Verbose Mode (-v, -vv, -vvv, & more)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::from_args();
    let cached = cache::fetch(&args.word).await;

    match cached {
        Ok(payload) => output::print(&payload, &args.word),
        Err(_) => {
            let response = request::request(&args.word).await;
            match response {
                Err(_) => 
                    output::print_not_found(&args.word),
                Ok(r) => {
                    cache::load(&parser::parse(&r)).await?;
                    output::print(&parser::parse(&r), &args.word);
                } 
            }
        }
    }
    
    Ok(())
}
