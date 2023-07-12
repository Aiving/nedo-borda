use serde::{Serialize, Deserialize};

#[derive(Serialize, Default, Clone, Deserialize, PartialEq)]
pub(crate) struct Post {
    pub(crate) id: i32,
    pub(crate) author: String,
    pub(crate) content: String,
}
