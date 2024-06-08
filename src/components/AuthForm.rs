use serde::Serialize;
use serde_wasm_bindgen::to_value;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use yew::platform::spawn_local;
use yew::prelude::*;

use crate::contexts::user::{UserAction, UserContextType};
use crate::gql::authenticate::authenticate;

#[derive(Serialize)]
struct SaveAuthTokenParams {
    token: String,
}

#[function_component(AuthForm)]
pub fn auth_form() -> Html {
    let user_context = use_context::<UserContextType>().expect("No AuthContext found");

    let username = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());

    let set_token = Rc::new({
        let user_context = user_context.clone();
        move |token: &String| {
            user_context.dispatch(UserAction::SetToken(token.clone()));
        }
    });

    let clear_token = {
        let user_context = user_context.clone();
        move || {
            user_context.dispatch(UserAction::ClearToken);
        }
    };

    let on_username_change = {
        let username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            username.set(input.value());
        })
    };

    let on_password_change = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            password.set(input.value());
        })
    };

    let on_submit = {
        let username = username.clone();
        let password = password.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let username = username.clone();
            let password = password.clone();
            let set_token = Rc::clone(&set_token); // Ensure set_token is cloned correctly

            spawn_local(async move {
                web_sys::console::log_1(
                    &format!("Username: {}, Password: {}", *username, *password).into(),
                );

                let auth_response = authenticate(&username, &password)
                    .await
                    .expect("Couldn't unwrap the auth response");
                let auth_token = auth_response.authenticate;

                web_sys::console::log_1(&format!("auth_response: {}", auth_token).into());

                let params = to_value(&SaveAuthTokenParams {
                    token: auth_token.clone(),
                })
                .unwrap();
                let result = crate::app::invoke("save_auth_token", params).await;

                set_token(&auth_token);

                // TODO: show files UI
            });
        })
    };

    html! {
        <form onsubmit={on_submit}>
            <div>
                <label for="username">{"Username:"}</label>
                <input
                    type="text"
                    id="username"
                    value={(*username).clone()}
                    oninput={on_username_change}
                />
            </div>
            <div>
                <label for="password">{"Password:"}</label>
                <input
                    type="password"
                    id="password"
                    value={(*password).clone()}
                    oninput={on_password_change}
                />
            </div>
            <button type="submit">{"Sign In"}</button>
        </form>
    }
}
