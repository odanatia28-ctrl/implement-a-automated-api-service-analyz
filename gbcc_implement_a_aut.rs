use reqwest;
use serde_json::json;
use std::collections::HashMap;

struct ApiAnalyzer {
    base_url: String,
}

impl ApiAnalyzer {
    fn new(base_url: &str) -> Self {
        ApiAnalyzer {
            base_url: base_url.to_string(),
        }
    }

    async fn analyze(&self, endpoint: &str) -> Result<(), reqwest::Error> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let res = reqwest::get(&url).await?;

        let status_code = res.status();
        let headers = res.headers().clone();
        let body = res.text().await?;

        println!("Endpoint: {}", endpoint);
        println!("Status Code: {}", status_code);
        println!("Headers: {:#?}", headers);
        println!("Body: {}", body);

        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let analyzer = ApiAnalyzer::new("https://api.example.com");

    let endpoints = vec!["users", "posts", "comments"];

    for endpoint in endpoints {
        analyzer.analyze(endpoint).await.unwrap();
    }
}