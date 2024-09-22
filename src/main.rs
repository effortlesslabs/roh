mod openai;
mod markdown;
mod config;

use openai::ask_openai;
use std::env;
use crate::config::Config;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "ask" if args.len() > 2 => {
            let question = &args[2];
            let config = Config::load().expect("API key not set. Please run 'roh config'");
            match ask_openai(&config, question).await {
                Ok(_) => {}
                Err(err) => eprintln!("Error fetching response: {}", err),
            }
        }
        "config" => {
            let api_key = args[2].clone();
            let base_uri = args[3].clone();
            let config = Config { api_key, base_uri };
            config.save();
        }
        "get-config" => {
            let config = Config::load().expect("No configuration found.");
            println!("API Key: {}\nBase URI: {}", config.api_key, config.base_uri);
        }
        _ => {
            eprintln!("Usage: roh <command>\nCommands: ask, config");
        }
    }
}
