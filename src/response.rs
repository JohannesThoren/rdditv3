use serde::{Serialize, Deserialize};
use crate::post::Post;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response {
    pub kind: String,
    pub data: RessponseData
}
#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct RessponseData {
    pub children: Vec<Post>,
}