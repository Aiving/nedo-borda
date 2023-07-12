use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Form, Json,
};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{entities::post::Post, ApplicationState};

#[derive(Deserialize)]
pub(crate) struct CreateThreadReplyPayload {
    pub(crate) message: String,
    pub(crate) author: String,
}

pub(crate) async fn create_thread_reply(
    Path(thread_id): Path<i32>,
    State(state): State<Arc<ApplicationState>>,
    Form(form): Form<CreateThreadReplyPayload>,
) -> Json<Value> {
    let mut threads = state.threads.lock().await;

    let thread = threads.iter_mut().find(|x| x.id == thread_id).unwrap();

    let last_post_id = thread
        .posts
        .as_ref()
        .unwrap()
        .last()
        .unwrap_or(&Post::default())
        .id;

    let post = Post {
        id: last_post_id + 1,
        author: form.author.clone(),
        content: form.message.clone(),
    };

    thread.posts.as_mut().unwrap().push(post.clone());

    Json(json!(post))
}
