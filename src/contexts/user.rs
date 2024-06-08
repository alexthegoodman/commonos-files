use std::rc::Rc;
use yew::functional::*;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct UserState {
    pub token: Option<String>,
}

pub enum UserAction {
    SetToken(String),
    ClearToken,
}

impl Default for UserState {
    fn default() -> Self {
        Self { token: None }
    }
}

impl Reducible for UserState {
    type Action = UserAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_ctr = match action {
            UserAction::SetToken(token) => UserState { token: Some(token) },
            UserAction::ClearToken => UserState { token: None },
        };

        Self { ..next_ctr }.into()
    }
}

pub type UserContextType = UseReducerHandle<UserState>;
