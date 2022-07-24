use serde::{Deserialize, Serializer};

#[derive(Deserialize)]
pub struct Post {
    pub subbreddit: String,
    pub selftext: String,
    pub selftext_html: String,
    pub author_fullname: String,
}
