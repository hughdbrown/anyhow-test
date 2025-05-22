use anyhow::{self, Context};
use reqwest;
use tokio;

async fn fetch_data(url: &str) -> anyhow::Result<String, anyhow::Error>
{
    let response = reqwest::get(url).await.context("Failed to fetch data")?;
    let text = response.text().await.context("Failed to parse response")?;
    Ok(text)
}

#[tokio::main]
async fn main() {
    let url: &str = &"https://google.com/asdasasd";
    let result = fetch_data(&url).await;
    match result {
         Ok(text) => println!("{}", text),
         Err(err) => println!("{}", err),
    }
}
