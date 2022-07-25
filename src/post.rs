use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]

pub struct Post {
    pub kind: String,
    pub data: Post_Data,
}
#[derive(Debug, Serialize, Deserialize)]

pub struct Post_Data {
    pub selftext: String,
    pub title: String,
    pub author: String,
}
