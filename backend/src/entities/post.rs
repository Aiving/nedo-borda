use serde::Serialize;

#[derive(Serialize, Default, Clone)]
pub(crate) struct Post {
    pub(crate) id: i32,
    pub(crate) author: String,
    pub(crate) content: String,
}
