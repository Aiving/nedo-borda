pub(crate) mod entities;
pub(crate) mod routes;

use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use entities::thread::Thread;
use tokio::sync::Mutex;

pub(crate) struct ApplicationState {
    pub(crate) threads: Mutex<Vec<Thread>>,
}

#[tokio::main]
async fn main() {
    let state = Arc::new(ApplicationState {
        threads: Mutex::new(vec![]),
    });

    let application = Router::new()
        .route(
            "/threads",
            post(routes::threads::create_thread).get(routes::threads::get_threads),
        )
        .route("/threads/:threadId", get(routes::threads::id::get_thread))
        .route(
            "/threads/:threadId/post",
            post(routes::threads::id::post::create_thread_reply),
        )
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(application.into_make_service())
        .await
        .unwrap();
}
