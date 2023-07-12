pub(crate) mod entities;

use entities::thread::Thread;
use gloo_net::http::Request;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct ThreadsListProps {
    threads: Vec<Thread>,
}

#[function_component(ThreadsList)]
fn threads_list(ThreadsListProps { threads }: &ThreadsListProps) -> Html {
    threads
        .iter()
        .map(|thread| {
            html! {
                <div class="thread" key={thread.id}>
                    <span class="title">{thread.name.clone()}</span>
                    <span class="post-author">{thread.initial_post.author.clone()}</span>
                    <span class="id">{"#"}{thread.id}</span>
                    <br />
                    <span class="post-content">{thread.initial_post.content.clone()}</span>
                </div>
            }
        })
        .collect()
}

#[function_component(App)]
fn app() -> Html {
    let threads = use_state(|| vec![]);

    {
        let threads = threads.clone();

        use_effect_with_deps(
            move |_| {
                let threads = threads.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_threads: Vec<Thread> =
                        Request::get("http://localhost:3000/threads")
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();

                    threads.set(fetched_threads);
                });

                || ()
            },
            (),
        );
    }

    html! {
        <>
            <div class="thread-create">
                <input type="text" id="author" value="Аноним" autocomplete="false" />
                <input type="text" id="name" autocomplete="false" />
                <textarea type="text" id="content" autocomplete="false" />
                <button>{"Создать тред"}</button>
            </div>
            <ThreadsList threads={(*threads).clone()} />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
