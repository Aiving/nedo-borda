use std::sync::Arc;

use axum::{extract::State, Json};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{
    entities::{post::Post, thread::Thread},
    ApplicationState,
};

pub(crate) mod id;

#[derive(Deserialize)]
pub(crate) struct CreateThreadPayload {
    pub(crate) name: String,
    pub(crate) author: String,
    pub(crate) content: String,
}

pub(crate) async fn get_threads(State(state): State<Arc<ApplicationState>>) -> Json<Value> {
    let threads = state.threads.lock().await;

    Json(json!(threads.iter().map(|thread| thread.without_posts()).collect::<Vec<Thread>>()))
}

pub(crate) async fn create_thread(
    State(state): State<Arc<ApplicationState>>,
    Json(payload): Json<CreateThreadPayload>,
) -> Json<Value> {
    let mut threads = state.threads.lock().await;

    let thread = Thread {
        id: threads.last().unwrap_or(&Thread::default()).id + 1,
        name: payload.name,
        initial_post: Post {
            id: 1,
            author: payload.author,
            content: payload.content,
        },
        posts: Some(vec![]),
    };

    threads.push(thread.clone());

    Json(json!(thread))
}
