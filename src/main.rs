mod openai;
mod markdown;
mod config;
mod bluetooth;
mod audio;

use openai::ask_openai;
use std::env;
use crate::config::Config;

#[tokio::main]
async fn main() {
    // Step 1: Discover and connect to Bluetooth headphones
    if let Err(e) = bluetooth::discover_connect_devices().await {
        eprintln!("Failed to connect to Bluetooth device: {:?}", e);
        return;
    }

    // Step 2: Play audio output
    // let audio_file_path = "assets/sample_speech.wav";
    if let Err(e) = audio::play_audio() {
        eprintln!("Failed to play audio: {:?}", e);
    }
}

// #[tokio::main]
// async fn main() {
//     let args: Vec<String> = env::args().collect();

//     match args[1].as_str() {
//         "ask" if args.len() > 2 => {
//             let question = &args[2];
//             let config = Config::load().expect("API key not set. Please run 'roh config'");
//             match ask_openai(&config, question).await {
//                 Ok(_) => {}
//                 Err(err) => eprintln!("Error fetching response: {}", err),
//             }
//         }
//         "config" => {
//             let api_key = args[2].clone();
//             let base_uri = args[3].clone();
//             let config = Config { api_key, base_uri };
//             config.save();
//         }
//         "get-config" => {
//             let config = Config::load().expect("No configuration found.");
//             println!("API Key: {}\nBase URI: {}", config.api_key, config.base_uri);
//         }
//         _ => {
//             eprintln!("Usage: roh <command>\nCommands: ask, config");
//         }
//     }
// }
