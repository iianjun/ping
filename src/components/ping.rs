use crate::utils::classnames;
use leptos::*;
#[component]
pub fn Ping(#[prop(optional)] class: Option<&'static str>) -> impl IntoView {
    view! {
        <span class=move || {
            classnames(
                &[
                    ("bg-secondary-green rounded-[50%] w-3 h-3 shadow-[0_0_0_4px_#141718]", true),
                    (class.unwrap_or(""), class.is_some()),
                ],
            )
        }></span>
    }
}
