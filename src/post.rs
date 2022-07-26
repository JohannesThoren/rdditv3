use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]

pub struct Post {
    pub kind: String,
    pub data: PostData,
}
#[derive(Debug, Serialize, Deserialize)]

pub struct PostData  {
    pub title: String,
    pub author: String,
    pub subreddit: String,
    pub selftext: Option<String>, 
    pub selftext_html: Option<String>,
    pub thumbnail: String,
    pub permalink: String,
    pub url: String,
    pub post_hint: String,
    pub ups: u32,
    pub downs: u32
}
