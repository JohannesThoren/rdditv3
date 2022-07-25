use serde::{Serialize, Deserialize};
use crate::post::Post;
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Subreddit {
    pub kind: String,
    pub data: Subreddit_Data
}
#[derive(Debug, Serialize, Deserialize)]

pub struct Subreddit_Data {
    pub children: Vec<Post>,
}