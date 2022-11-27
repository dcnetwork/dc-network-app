use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
// use yew::{html, Children, Component, Context, Html, Properties, function_component};
use gloo_timers::callback::Timeout;
use web_sys::window;

// ************* my libs *************
use dc_network_app_ui::components::welcome::*;
use dc_network_app_ui::theme::Theme;
use dc_network_app_ui::components::side_navbar::*;
use dc_network_app_ui::components::context_option::ThemedContextOption;
use dc_network_app_ui::components::context::ThemedContext;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// #[derive(Serialize, Deserialize)]
// struct GreetArgs<'a> {
//     name: &'a str,
// }

// #[function_component(App)]
// pub fn app() -> Html {
//     let greet_input_ref = use_ref(|| NodeRef::default());

//     let name = use_state(|| String::new());

//     let greet_msg = use_state(|| String::new());
//     {
//         let greet_msg = greet_msg.clone();
//         let name = name.clone();
//         let name2 = name.clone();
//         use_effect_with_deps(
//             move |_| {
//                 spawn_local(async move {
//                     if name.is_empty() {
//                         return;
//                     }

//                     // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//                     let new_msg = invoke(
//                         "greet",
//                         to_value(&GreetArgs { name: &*name }).unwrap(),
//                     )
//                     .await;
//                     log(&new_msg.as_string().unwrap());
//                     greet_msg.set(new_msg.as_string().unwrap());
//                 });

//                 || {}
//             },
//             name2,
//         );
//     }

//     let greet = {
//         let name = name.clone();
//         let greet_input_ref = greet_input_ref.clone();
//         Callback::from(move |_| {
//             name.set(greet_input_ref.cast::<web_sys::HtmlInputElement>().unwrap().value());
//         })
//     };

//     html! {
//         <main class="container">
//             <div class="row">
//                 <a href="https://tauri.app" target="_blank">
//                     <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
//                 </a>
//                 <a href="https://yew.rs" target="_blank">
//                     <img src="public/yew.png" class="logo yew" alt="Yew logo"/>
//                 </a>
//             </div>

//             <p>{"Click on the Tauri and Yew logos to learn more."}</p>

//             <p>
//                 {"Recommended IDE setup: "}
//                 <a href="https://code.visualstudio.com/" target="_blank">{"VS Code"}</a>
//                 {" + "}
//                 <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank">{"Tauri"}</a>
//                 {" + "}
//                 <a href="https://github.com/rust-lang/rust-analyzer" target="_blank">{"rust-analyzer"}</a>
//             </p>

//             <div class="row">
//                 <input id="greet-input" ref={&*greet_input_ref} placeholder="Enter a name..." />
//                 <button type="button" onclick={greet}>{"Greet"}</button>
//             </div>

//             <p><b>{ &*greet_msg }</b></p>
//         </main>
//     }
// }



#[function_component(App)]
pub fn app() -> Html {
    let splash = use_state(|| true);
    let csplash = splash.clone(); // cloned splash
    
    let timeout = Timeout::new(3000, move || {
        csplash.set(false);
    });    

    timeout.forget();

    let ctx = use_state(|| Theme {
        foreground: "#ffffff".to_owned(),
        background: "#192035".to_owned(),
    });

    html! {
        if *splash{
            <ContextProvider<Theme> context={(*ctx).clone()}>
                <ThemedWelcome />
            </ContextProvider<Theme>>
        }
        else{
            <ContextProvider<Theme> context={(*ctx).clone()}>
                <ThemedNavbar />
                <ThemedContextOption />
                <ThemedContext />

            </ContextProvider<Theme>>   
        }
    }

}
