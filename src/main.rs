use std::io;
use std::io::Write;
use clap::{Parser, ValueEnum};
use clap::builder::PossibleValue;
use colored::*;

mod utils;
mod api;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Models {
    GPT35,
    GPT4
}

impl ValueEnum for Models {
    fn value_variants<'a>() -> &'a [Self] {
        &[Models::GPT35, Models::GPT4]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            Models::GPT35 => PossibleValue::new("gpt3.5").help("Use the GPT-3.5 model"),
            Models::GPT4 => PossibleValue::new("gpt4").help("Use the newer and more expensive GPT-4 model"),
        })
    }
}

impl std::fmt::Display for Models {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl std::str::FromStr for Models {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("invalid variant: {}", s))
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = Models::GPT35)]
    model: Models,

    #[arg(long, default_value = "1000")]
    max_tokens: u32
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    println!("{} {}.", "Welcome to chadgpt! Running version".bright_green(), env!("CARGO_PKG_VERSION"));
    let secret_opt = utils::get_secret();
    let mut key = String::new();
    if secret_opt.is_none() {
        println!("First time setup. Please enter your OpenAI API key.");
        io::stdin().read_line(&mut key).expect("Failed to read line");
        utils::write_secret(&key).expect("Failed to write secret");
    }
    else {
        key = secret_opt.unwrap()
    }
    let mut api = api::API::new(key, args.model, args.max_tokens);
    loop {
        print!("User: ");
        io::stdout().flush().unwrap();
        let mut message = String::new();
        io::stdin().read_line(&mut message).expect("Failed to read line");
        let response = api.send(&message).await;
        println!("AI: {}", response);
    }
}
