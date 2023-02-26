use reqwest::{header, Client};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Repository {
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RawResponse {
    items: Vec<Repository>,
}

pub async fn search_repositories(token: String, search_query: String) -> Vec<Repository> {
    println!("Searching for {}", search_query);
    let client = Client::new();

    let response = client
        .get("https://api.github.com/search/repositories")
        .header(header::USER_AGENT, "rust-lang")
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .query(&[("q", format!("user:oacs {}", search_query))])
        .send()
        .await;

    response.unwrap().json::<RawResponse>().await.unwrap().items
}
