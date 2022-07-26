use std::future::Future;

use crate::{post::Post, response};

pub async fn fetch(url: String) -> Vec<Post> {
    match reqwest::get(url).await {
        Ok(response) => {
            let response_text = response.text().await.unwrap();
            let response_data: response::Response = serde_json::from_str(&response_text.as_str())
                .expect("JSON does not have correct format");

            return response_data.data.children;
        }
        _ => {
            return vec![];
        }
    };
}
