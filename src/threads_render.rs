use horrorshow::html;
use horrorshow::helper::doctype;

pub fn render_threads(threads: &Vec<crate::threads::Thread>) -> std::string::String {
    format!("{}", html! {
        : doctype::HTML;
        html {
            head {
                title : "Render html test";
                meta(charset="UTF-8");
                style {
                    : "h3 {";
                        : "white-space: pre-wrap;";
                    : "}";
                }
            }
            body {
                h1 : "Список тредов";
                ol {
                    @ for thread in threads {
                        li {
                            h2 : format!("{} (Автор: {})", thread.name.to_owned(), thread.initial_post.author.to_owned());
                            h3 : thread.initial_post.content.to_owned();
                            button(onclick=format!("document.location.pathname='/threads/{}'", thread.id)) : "Перейти к треду";
                        }
                    }
                }
                h2 : "Создание треда";
                form(action="/api/threads", method="post") {
                    label(for="author") : "Имя анонимуса:"; br;
                    input(type="text", id="author", name="author", value="Аноним"); br;
                    label(for="name") : "Имя поста"; br;
                    input(type="text", id="name", name="name"); br;
                    label(for="initial_content") : "Контент поста:"; br;
                    textarea(type="text", id="initial_content", name="initial_content"); br;
                    input(type="submit", value="Создать тред");
                }
            }
        }
    })
}
pub fn render_thread(thread: &crate::threads::Thread) -> std::string::String {
    format!("{}", html! {
        : doctype::HTML;
        html {
            head {
                title : "Render html test";
                meta(charset="UTF-8");
                style {
                    : "a {";
                        : "white-space: pre-wrap;";
                    : "}";
                }
            }
            body {
                h1 : thread.name.to_owned();
                b : format!("Написал: {}", thread.initial_post.author.to_owned());
                br; br;
                a : thread.initial_post.content.to_owned();
                ol {
                    @ for post in thread.posts.iter() {
                        li {
                            b : post.author.to_owned();
                            br;
                            a : post.content.to_owned();
                        }
                    }
                }
                form(action=format!("/api/threads/{id}/post", id=thread.id), method="post") {
                    label(for="author") : "Имя анонимуса:"; br;
                    input(type="text", id="author", name="author", value="Аноним"); br;
                    label(for="message") : "Отправить ответ:"; br;
                    textarea(type="text", id="message", name="message"); br;
                    input(type="submit", value="Отправить");
                }
            }
        }
    })
}