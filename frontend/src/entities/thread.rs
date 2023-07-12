use serde::{Serialize, Deserialize};

use super::post::Post;

#[derive(Serialize, Default, Clone, Deserialize, PartialEq)]
pub(crate) struct Thread {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) initial_post: Post,
    pub(crate) posts: Option<Vec<Post>>,
}