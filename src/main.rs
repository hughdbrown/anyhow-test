use anyhow::{self, Context};
use reqwest;
use tokio;

type AnyhowError = anyhow::Result<String, anyhow::Error>;

async fn fetch_data(url: &str) -> AnyhowError
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

#[tokio::test]
async fn test_fetch_data_error() {
    let url = &"https://google.com/asdasd";
    let result: AnyhowError = fetch_data(&url).await;

    match result {
        Ok(x) => println!("Not an error: {}", x),
        Err(_) => assert!(result.is_err()),
    }
}

#[tokio::test]
async fn test_fetch_data_no_error() {
    let url = &"https://google.com/";
    let result: AnyhowError = fetch_data(&url).await;

    match result {
        Ok(_) => assert!(! result.is_err()),
        Err(x) => println!("Unexpected error: {}", x),
    }
}
