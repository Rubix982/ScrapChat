mod models;
mod constants;
mod utils;

use std::error::Error;
use reqwest::blocking::Client;
use scraper::Html;
use serde_json::Value;
use std::fs::create_dir_all;
use std::io::Write;
use std::process;
use url::Url;

fn log_time(msg: &str) {
    println!("[{}] {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), msg);
}

fn handle_gpt_response(url: &str) -> Result<(), Box<dyn Error>> {
    let response = Client::new().get(url).send()?;

    if !response.status().is_success() {
        log_time(&format!("Failed to fetch the URL. Status code: {}", response.status()));
        process::exit(1);
    }

    let body = response.text()?;
    let document = Html::parse_document(&body);
    let json_data = utils::extract_json_data(&utils::extract_chat_script_contents(&document)?)?;
    let parsed_data: Value = serde_json::from_str(json_data)?;
    let linear_conversation = utils::get_linear_conversation(parsed_data);
    let output_dir = format!("chat/{}", utils::extract_title(&document)?);

    create_dir_all(&output_dir)?;
    utils::save_content(&output_dir, &body, &linear_conversation)?;

    log_time(&format!("Content saved to '{}'", output_dir));

    Ok(())
}

fn main() {
    print!("Enter the URL: ");
    let mut url_input = String::new();
    std::io::stdin().read_line(&mut url_input).unwrap();
    let url_input = url_input.trim();

    let url_constant = constants::CHAT_GPT_URL_SHARE_CONSTANT;
    if !url_input.starts_with(url_constant) {
        log_time(&format!("Provided URL does not start with '{}'. Exiting.", url_constant));
        process::exit(1);
    }

    if let Err(e) = Url::parse(url_input) {
        log_time(&format!("Invalid URL: {}", e));
        process::exit(1);
    }

    if let Err(e) = handle_gpt_response(url_input) {
        log_time(&format!("Error: {}", e));
        process::exit(1);
    }
}
