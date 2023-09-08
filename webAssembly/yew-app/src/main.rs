pub mod components;
pub mod list;
use crate::components::VideoList;
use gloo_net::http::Request;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Video {
    pub id: usize,
    pub title: String,
    pub speaker: String,
    pub url: String,
}

#[function_component(App)]
fn app() -> Html {
    let on_get_img = Callback::from(move |ev: FocusEvent| {
        ev.prevent_default();
        wasm_bindgen_futures::spawn_local(async move {
            let res = Request::post("http://localhost:8080/")
                .send()
                .await
                .unwrap();

            assert_eq!(res.status(), 200);
            // println!("success: {}", res.json().await);
        });
    });

    html! {
    <>
           <h1>{ "RustConf Explorer" }</h1>
           <div>
               <h3>{"Videos to watch"}</h3>
                <VideoList videos={list::generate_playlist()}/>
           </div>
       <button on_click={on_get_img}>
        { "预览" }
        </button>
       </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
