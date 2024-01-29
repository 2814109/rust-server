use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use axum::{response::IntoResponse, Json};
use axum::{routing::get, Router};
use dotenv::dotenv;
use reqwest::Client;
use std::env;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    // build our application with a route
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = Router::new().route("/api/github", get(handler)).layer(cors);

    println!("🚀 Server started successfully");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub async fn handler() -> axum::response::Result<impl IntoResponse> {
    dotenv().ok();
    let url = env::var("GITHUB_API").expect("URL must be set");
    let client: Client = Client::new();
    let response = client
        .get(url)
        .header(
            reqwest::header::USER_AGENT,
            "Mozilla/5.0 (platform; rv:geckoversion) Gecko/geckotrail Firefox/firefoxversion",
        )
        .send()
        .await
        .unwrap();

    let body = response.text().await.unwrap();
    // const MESSAGE: &str = "Build Simple CRUD API in Rust using Axum";

    let json_response = serde_json::json!({
        "status": "success",
        "message": body
    });

    Ok(Json(json_response))
}
