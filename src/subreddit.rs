use crate::post::{self, Post};
pub struct Subreddit {
    after: String,
    dist: u32,
    modhash: String,
    children: Vec<Post>
}