// use axum_macros::debug_handler;
use dotenv::dotenv;
use reqwest::Client;
use std::env;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let url = env::var("GITHUB_API").expect("URL must be set");
    // let url = "https://zipcloud.ibsnet.co.jp/api/search";
    println!("{}", url);
    let client: Client = Client::new();
    let response = client
        .get(url)
        .header(
            reqwest::header::USER_AGENT,
            "Mozilla/5.0 (platform; rv:geckoversion) Gecko/geckotrail Firefox/firefoxversion",
        )
        .send()
        .await?;

    let body = response.text().await?;
    println!("{}", body);

    Ok(())
}
