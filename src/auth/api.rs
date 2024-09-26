use leptos::*;
#[derive(serde::Deserialize, Clone, serde::Serialize)]
pub enum SignupResponse {
    ValidationError(String),
    CreateUserError(String),
    Success,
}

#[server(SignupAction, "/api")]
pub async fn signup_action(email: String, password: String) -> Result<bool, ServerFnError> {
    println!("{:?}", email);
    println!("{:?}", password);
    Ok(true)
}
#[server(LoginAction, "/api")]
pub async fn login_action(email: String, password: String) -> Result<bool, ServerFnError> {
    println!("{:?}", email);
    println!("{:?}", password);
    Ok(true)
}
