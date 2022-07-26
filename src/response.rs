use serde::{Serialize, Deserialize};
use crate::post::Post;
#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub kind: String,
    pub data: RessponseData
}
#[derive(Debug, Serialize, Deserialize)]

pub struct RessponseData {
    pub children: Vec<Post>,
}