use yew::platform::spawn_local;
use yew::prelude::*;

use crate::gql::authenticate::authenticate;

#[function_component(AuthForm)]
pub fn auth_form() -> Html {
    let username = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());

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
            spawn_local(async move {
                web_sys::console::log_1(
                    &format!("Username: {}, Password: {}", *username, *password).into(),
                );

                let auth_response = authenticate(&*username, &*password)
                    .await
                    .expect("Couldn't unwrap the auth response");

                web_sys::console::log_1(
                    &format!("auth_response: {}", auth_response.authenticate).into(),
                );
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
