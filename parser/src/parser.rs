use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::error::Error;

fn parse(url: &str) -> Result<String, Box<dyn Error>> {
    let response = get(url)?.text()?;
    
    // Parse the HTML response
    let document = Html::parse_document(&response);
    
    // Create a selector for script tags
    let script_selector = Selector::parse("script").unwrap();
    
    // Find the script tag that contains 'articleBody'
    for element in document.select(&script_selector) {
        if let Some(script_text) = element.text().next() {
            if script_text.contains("articleBody") {
                // Extract the description and articleBody content
                let desc_start = script_text.find(r#""description":"#).ok_or("No description found")?;
                let desc_end = script_text.find(r#""genre""#).ok_or("No genre found")?;
                let desc = &script_text[desc_start + 14..desc_end];

                let body_start = script_text.find("articleBody").ok_or("No articleBody found")?;
                let body_end = script_text.find('@').unwrap_or(script_text.len());
                let body = &script_text[body_start + 12..body_end];

                return Ok(format!("{}{}", desc, body));
            }
        }
    }
