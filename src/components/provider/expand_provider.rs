use leptos::*;

#[component]
pub fn ExpandProvider(children: Children) -> impl IntoView {
    provide_context(RwSignal::new(true));
    view! { <>{children()}</> }
}
