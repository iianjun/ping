use leptos::{Action, ServerFnError};
mod api;
pub use api::*;
// pub type LogoutSignal = Action<LogoutAction, Result<(), ServerFnError>>;
// pub type LoginSignal = Action<LoginAction, Result<LoginMessages, ServerFnError>>;
pub type SignupSignal = Action<SignupAction, Result<bool, ServerFnError>>;
pub type LoginSignal = Action<LoginAction, Result<bool, ServerFnError>>;
