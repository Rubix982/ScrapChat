// MIT License
//
// Copyright (c) 2024 Saif Ul Islam (Rubix982)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

mod models;
mod constants;
mod utils;

use std::error::Error;
use reqwest::blocking::Client;
use scraper::Html;
use serde_json::Value;
use std::fs::create_dir_all;
use std::{env, process};
use url::Url;

fn handle_gpt_response(url: &str) -> Result<(), Box<dyn Error>> {
    let response = Client::new().get(url).send()?;

    if !response.status().is_success() {
        utils::log_time(&format!("Failed to fetch the URL. Status code: {}", response.status()));
        process::exit(1);
    }

    let body = response.text()?;
    let document = Html::parse_document(&body);
    let script_contents = utils::extract_chat_script_contents(&document)?;
    let json_data = utils::extract_json_data(&script_contents)?;
    let parsed_data: Value = serde_json::from_str(json_data)?;
    let linear_conversation = utils::get_linear_conversation(parsed_data);
    let output_dir = format!("chat/{}", utils::extract_title(&document)?);

    create_dir_all(&output_dir)?;
    utils::save_content(&output_dir, &linear_conversation)?;

    utils::log_time(&format!("Content saved to '{}'", output_dir));

    Ok(())
}

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure that the correct number of arguments is passed (URL as the only argument)
    if args.len() != 2 {
        utils::log_time("Usage: cargo run -- <URL>");
        process::exit(1);
    }

    // The URL is the second argument (args[1])
    let url_input = &args[1];

    // Validate the URL
    let url_constant = constants::CHAT_GPT_URL_SHARE_CONSTANT;
    if !url_input.starts_with(url_constant) {
        utils::log_time(&format!("Provided URL does not start with '{}'. Exiting.", url_constant));
        process::exit(1);
    }

    // Check if the URL is valid
    if let Err(e) = Url::parse(url_input) {
        utils::log_time(&format!("Invalid URL: {}", e));
        process::exit(1);
    }

    // Process the URL
    if let Err(e) = handle_gpt_response(url_input) {
        utils::log_time(&format!("Error: {}", e));
        process::exit(1);
    }
}
