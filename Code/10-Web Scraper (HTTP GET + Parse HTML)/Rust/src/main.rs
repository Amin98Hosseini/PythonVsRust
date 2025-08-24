use reqwest;
use scraper::{Html, Selector};
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://en.wikipedia.org/wiki/Rust_(programming_language)";

    // --- HTTP GET ---
    let start = Instant::now();
    let response = reqwest::get(url).await?;
    let dur_get = start.elapsed().as_micros();
    println!("HTTP GET completed in {} µs, Status: {}", dur_get, response.status());

    let start = Instant::now();
    let body = response.text().await?;
    let document = Html::parse_document(&body);
    let selector = Selector::parse("h1").unwrap();
    let titles: Vec<_> = document.select(&selector).map(|t| t.inner_html()).collect();
    let dur_parse = start.elapsed().as_micros();
    println!("HTML Parsing completed in {} µs, Found {} h1 tags", dur_parse, titles.len());

    if let Some(first) = titles.first() {
        println!("First h1 tag: {}", first);
    } else {
        println!("No h1 tag found");
    }

    Ok(())
}
