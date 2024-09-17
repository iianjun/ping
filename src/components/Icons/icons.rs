use leptos::*;

#[component]
pub fn SidebarIcon(#[prop(optional)] class: &'static str) -> impl IntoView {
    // Using `CARGO_MANIFEST_DIR` to define the absolute path
    let src = include_str!("../../../assets/svgs/sidebar.svg");

    view! { <span class=class inner_html=src /> }
}

#[component]
pub fn ChatIcon(#[prop(optional)] class: &'static str) -> impl IntoView {
    // Using `CARGO_MANIFEST_DIR` to define the absolute path
    let src = include_str!("../../../assets/svgs/chat.svg");

    view! { <span class=class inner_html=src /> }
}

#[component]
pub fn GroupIcon(#[prop(optional)] class: &'static str) -> impl IntoView {
    // Using `CARGO_MANIFEST_DIR` to define the absolute path
    let src = include_str!("../../../assets/svgs/group.svg");

    view! { <span class=class inner_html=src /> }
}

#[component]
pub fn SearchIcon(#[prop(optional)] class: &'static str) -> impl IntoView {
    // Using `CARGO_MANIFEST_DIR` to define the absolute path
    let src = include_str!("../../../assets/svgs/search.svg");

    view! { <span class=class inner_html=src /> }
}

#[component]
pub fn LogoutIcon(#[prop(optional)] class: &'static str) -> impl IntoView {
    // Using `CARGO_MANIFEST_DIR` to define the absolute path
    let src = include_str!("../../../assets/svgs/logout.svg");

    view! { <span class=class inner_html=src /> }
}
