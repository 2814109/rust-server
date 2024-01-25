use axum::{extract::Request, response::Html, routing::get, Router};
use std::convert::Infallible;

use dotenv::dotenv;
use reqwest::Client;
use std::env;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route(
        "/",
        get(
            |req: Request| async move {
                let res = handler().await?;
                Ok::<_, Infallible>(res)
            }, // || async { handler }, // handler.await?
        ),
    );

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Result<Html<String>> {
    dotenv().ok();
    let url = env::var("GITHUB_API").expect("URL must be set");
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
    Ok(Html(body))
    // Ok((Html(&body)))
}
// use dotenv::dotenv;
// use reqwest::Client;
// use std::env;

// #[tokio::main]
// async fn main() -> Result<()> {
//     dotenv().ok();
//     let url = env::var("GITHUB_API").expect("URL must be set");
//     // let url = "https://zipcloud.ibsnet.co.jp/api/search";
//     println!("{}", url);
//     let client: Client = Client::new();
//     let response = client
//         .get(url)
//         .header(
//             reqwest::header::USER_AGENT,
//             "Mozilla/5.0 (platform; rv:geckoversion) Gecko/geckotrail Firefox/firefoxversion",
//         )
//         .send()
//         .await?;
//     // let response = client
//     //     .get(url)
//     //     .query(&[("zipcode", "1000002")])
//     //     .send()
//     //     .await?;
//     let body = response.text().await?;
//     println!("{}", body);

//     Ok(())
// }
