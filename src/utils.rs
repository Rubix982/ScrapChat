use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use scraper::{Html, Selector};
use serde_json::Value;

pub(crate) fn get_linear_conversation(parsed_data: Value) -> String {
    parsed_data["state"]["loaderData"]["routes/share.$shareId.($action)"]
        ["serverResponse"]["data"]["linear_conversation"].to_string()
}

pub(crate) fn extract_json_data(chat_script_contents: &str) -> Result<&str, Box<dyn Error>> {
    let prefix = "window.__remixContext = ";
    let json_start = chat_script_contents.find(prefix).map(|p| p + prefix.len());

    if let Some(start) = json_start {
        Ok(&chat_script_contents[start..])
    } else {
        Err("JSON prefix not found in script contents".into())
    }
}

pub(crate) fn save_content(output_dir: &str, body: &str, route: &str) -> Result<(), Box<dyn Error>> {
    let html_path = Path::new(output_dir).join("main.html");
    let mut html_file = File::create(&html_path)?;
    html_file.write_all(body.as_bytes())?;

    let json_path = Path::new(output_dir).join("script.json");
    let mut json_file = File::create(&json_path)?;
    json_file.write_all(route.as_bytes())?;

    Ok(())
}

pub(crate) fn extract_title(document: &Html) -> Result<String, Box<dyn Error>> {
    let title_selector = Selector::parse("title")?;
    let title = document
        .select(&title_selector)
        .next()
        .ok_or("Title not found")?
        .inner_html()
        .replace("ChatGPT - ", "");
    Ok(title)
}

pub(crate) fn extract_chat_script_contents(document: &Html) -> Result<String, Box<dyn Error>> {
    let script_selector = Selector::parse("script")?;
    let scripts: Vec<_> = document.select(&script_selector).collect();

    if scripts.len() < 3 {
        return Err("Not enough <script> tags found".into());
    }

    let chat_script_contents = scripts[2].inner_html();
    let position = chat_script_contents
        .find(";__remixContext.p = function")
        .unwrap_or_else(|| chat_script_contents.len());
    Ok(chat_script_contents[..position].to_string())
}
