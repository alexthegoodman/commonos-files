use yew::prelude::*;

use crate::components::AuthForm::AuthForm;
use crate::components::SocketExample::SocketExample;
use crate::contexts::user::{UserAction, UserContextType, UserState};

#[function_component(Main)]
pub fn main() -> Html {
    let user_context = use_context::<UserContextType>().expect("No AuthContext found");
    let user_token = user_context.token.clone();

    html! {
        <main class="container">
            <h1>{"Welcome"}</h1>
            <span>{"Your token: "}{user_context.token.clone()}</span>
            if user_token == None {
                <>
                    <p>{"To get started with any CommonOS app, you need to be signed in. You also have to pick a folder to sync your files to!"}</p>
                    <AuthForm />
                </>
            } else {
                <SocketExample />
            }
        </main>
    }
}
