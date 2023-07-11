use actix_web::{get, post, web, Responder, HttpResponse};
use serde::{Serialize, Deserialize};
use crate::{posts::Post, threads_render::render_threads, threads_render::render_thread, AppState};

#[derive(Serialize)]
pub struct Thread {
    pub id: i32,
    pub name: String,
    pub initial_post: Post,
    pub posts: Vec<Post>
}

#[derive(Deserialize)]
pub struct ThreadPostBody {
    pub message: String,
    pub author: String
}

#[derive(Deserialize)]
pub struct ThreadCreateBody {
    pub name: String,
    pub initial_content: String,
    pub author: String
}

#[get("/threads")]
pub async fn get_threads(data: web::Data<AppState>) -> impl Responder {
    let threads = data.threads.lock().unwrap();

    HttpResponse::Ok().json(&*threads)
}

#[get("/threads")]
pub async fn get_rendered_threads(data: web::Data<AppState>) -> impl Responder {
    let threads = data.threads.lock().unwrap();

    HttpResponse::Ok().body(render_threads(&*threads))
}

#[get("/threads/{threadId}")]
pub async fn get_rendered_thread(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let thread_id = path.into_inner().parse::<i32>().unwrap();
    let threads = data.threads.lock().unwrap();
    let thread = threads.iter().find(|&x| x.id == thread_id);
    if thread.is_none() {
        return HttpResponse::Found().append_header(("Location", format!("/threads"))).finish();
    }
    let thread = thread.as_deref().unwrap();

    HttpResponse::Ok().body(render_thread(&thread))
}

#[post("/threads/{threadId}/post")]
pub async fn post_thread_reply(
    path: web::Path<String>, 
    data: web::Data<AppState>, 
    form: web::Form<ThreadPostBody>
) -> impl Responder {
    let thread_id = path.into_inner().parse::<i32>().unwrap();
    let mut threads = data.threads.lock().unwrap();
    let thread = &mut threads.iter_mut().find(|x| x.id == thread_id);
    if thread.is_none() {
        return HttpResponse::Found().append_header(("Location", format!("/threads"))).finish();
    }
    let thread = thread.as_deref_mut().unwrap();
    let last_post_id = thread.posts.iter().last().unwrap_or(&Post {
        id: 0,
        author: "".to_string(),
        content: "".to_string()
    }).id + 1;

    (*thread).posts.push(Post {
        id: last_post_id + 1,
        author: form.author.to_owned(),
        content: form.message.to_owned()
    });

    HttpResponse::Found().append_header(("Location", format!("/threads/{thread_id}"))).finish()
}

#[post("/threads")]
pub async fn post_thread(
    data: web::Data<AppState>, 
    form: web::Form<ThreadCreateBody>
) -> impl Responder {
    let mut threads = data.threads.lock().unwrap();
    let thread_id = threads.iter().last().unwrap().id + 1;
    (*threads).push(Thread {
        id: thread_id,
        name: form.name.to_owned(),
        initial_post: Post {
            id: 1,
            author: form.author.to_owned(),
            content: form.initial_content.to_owned()
        },
        posts: vec![]
    });
    HttpResponse::Found().append_header(("Location", format!("/threads/{thread_id}"))).finish()
}