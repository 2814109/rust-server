use dotenv::dotenv;
use reqwest::Client;
// use std::env;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    // let url = env::var("URL").expect("URL must be set");
    let url = "https://zipcloud.ibsnet.co.jp/api/search";
    println!("{}", url);
    let client: Client = Client::new();
    let response = client
        .get(url)
        .query(&[("zipcode", "1000002")])
        .send()
        .await?;
    let body = response.text().await?;
    println!("{}", body);

    Ok(())
}
