use serde::Serialize;

use super::post::Post;

#[derive(Serialize, Default, Clone)]
pub(crate) struct Thread {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) initial_post: Post,
    pub(crate) posts: Option<Vec<Post>>,
}

impl Thread {
    pub(crate) fn without_posts(&self) -> Thread {
        Thread {
            id: self.id,
            name: self.name.clone(),
            initial_post: self.initial_post.clone(),
            posts: None,
        }
    }
}
