use std::sync::Arc;

use axum::{Json, extract::{Path, State}};
use serde_json::{Value, json};

use crate::ApplicationState;

pub(crate) mod post;

pub(crate) async fn get_thread(Path(thread_id): Path<i32>, State(state): State<Arc<ApplicationState>>) -> Json<Value> {
    let threads = state.threads.lock().await;
    let thread = threads.iter().find(|thread| thread.id == thread_id);

    Json(json!(thread))
}