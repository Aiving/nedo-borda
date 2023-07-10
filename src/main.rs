mod threads_render;
mod threads;
mod posts;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use env_logger::{Env};
use posts::Post;
use crate::threads::Thread;
use std::sync::Mutex;

pub struct AppState {
    threads: Mutex<Vec<Thread>>
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let state = web::Data::new(AppState {
        threads: Mutex::new(vec![
            Thread {
                id: 1,
                name: "test".to_string(),
                initial_post: Post {
                    id: 1,
                    author: "Keneshin".to_string(),
                    content: "Йобана тестовый пост".to_string()
                },
                posts: vec![Post {
                    id: 2,
                    author: "Kurays".to_string(),
                    content: "Тест ответ".to_string()
                }]
            }
        ])
    });
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(state.clone())
            .service(hello)
            .service(threads::get_rendered_threads)
            .service(threads::get_rendered_thread)
            .service(web::scope("api")
                .service(threads::get_threads)
                .service(threads::post_thread_reply)
                .service(threads::post_thread)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}