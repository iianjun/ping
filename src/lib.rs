pub mod app;
pub mod components;
use std::fmt::Write;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    use leptos::*;

    console_error_panic_hook::set_once();

    mount_to_body(App);
}

pub fn classnames(classes: &[(&str, bool)]) -> String {
    let mut result = String::new();
    for &(class, condition) in classes {
        if condition {
            if !result.is_empty() {
                write!(result, " ").unwrap();
            }
            write!(result, "{}", class).unwrap();
        }
    }
    result
}
