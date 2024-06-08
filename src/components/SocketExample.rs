use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{MessageEvent, WebSocket};
use yew::functional::*;
use yew::prelude::*;

use crate::contexts::user::{UserAction, UserContextType};

#[function_component(SocketExample)]
pub fn socket_example() -> Html {
    let user_context = use_context::<UserContextType>().expect("No AuthContext found");

    // Use state to hold the WebSocket connection
    let ws = use_state(|| None);
    let ws_ref = ws.clone();

    // Effect to establish WebSocket connection
    use_effect_with((), move |_| {
        let ws = WebSocket::new("ws://localhost:4000").unwrap();

        let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
            if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                // log!(format!("Received message: {}", txt));
                web_sys::console::info_1(&format!("Received message: {}", txt).into());
            }
        });

        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();

        // Update state with the WebSocket connection
        ws_ref.set(Some(ws));

        // || () // Cleanup function if needed
    });

    // Function to send a message
    let send_message = {
        let ws = ws.clone();
        Callback::from(move |_| {
            if let Some(ws) = &*ws {
                let msg = serde_json::json!({
                    "Authorization": "Bearer ".to_owned() + &user_context.token.clone().expect("Token not found during socket message"),
                    "event": "eventName1",
                    "payload": "Hello"
                })
                .to_string();
                ws.send_with_str(&msg).unwrap();
            }
        })
    };

    // Function to disconnect
    let disconnect = {
        let ws = ws.clone();
        Callback::from(move |_| {
            if let Some(ws) = &*ws {
                ws.close().unwrap();
                // ws.set(None);
            }
        })
    };

    html! {
        <>
            <button onclick={send_message}>{ "Send Message" }</button>
            <button onclick={disconnect}>{ "Disconnect" }</button>
        </>
    }
}
