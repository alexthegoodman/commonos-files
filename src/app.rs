use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::components::Main::Main;
use crate::contexts::user::{UserAction, UserContextType, UserState};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[function_component(App)]
pub fn app() -> Html {
    // Initialize the state with the reducer
    let user_state = use_reducer(UserState::default);

    html! {
        <ContextProvider<UserContextType> context={user_state}>
            <Main />
        </ContextProvider<UserContextType>>
    }
}

// #[function_component(App)]
// pub fn app() -> Html {
//     let greet_input_ref = use_node_ref();

//     let name = use_state(|| String::new());

//     let greet_msg = use_state(|| String::new());
//     {
//         let greet_msg = greet_msg.clone();
//         let name = name.clone();
//         let name2 = name.clone();
//         use_effect_with(
//             name2,
//             move |_| {
//                 spawn_local(async move {
//                     if name.is_empty() {
//                         return;
//                     }

//                     let args = to_value(&GreetArgs { name: &*name }).unwrap();
//                     // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//                     let new_msg = invoke("greet", args).await.as_string().unwrap();
//                     greet_msg.set(new_msg);
//                 });

//                 || {}
//             },
//         );
//     }

//     let greet = {
//         let name = name.clone();
//         let greet_input_ref = greet_input_ref.clone();
//         Callback::from(move |e: SubmitEvent| {
//             e.prevent_default();
//             name.set(
//                 greet_input_ref
//                     .cast::<web_sys::HtmlInputElement>()
//                     .unwrap()
//                     .value(),
//             );
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

//             <form class="row" onsubmit={greet}>
//                 <input id="greet-input" ref={greet_input_ref} placeholder="Enter a name..." />
//                 <button type="submit">{"Greet"}</button>
//             </form>

//             <p><b>{ &*greet_msg }</b></p>
//         </main>
//     }
// }
