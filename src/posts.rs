use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub author: String,
    pub content: String,
}