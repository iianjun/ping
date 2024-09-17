use crate::utils::classnames;
use leptos::*;
use leptos_router::use_location;
#[component]
pub fn TabLink<F, IV>(
    href: &'static str,
    text: &'static str,
    icon: F,
    expand: ReadSignal<bool>,
) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    let location = use_location();
    let pathname = move || location.pathname.get();
    let is_active = move || pathname().starts_with(href);
    view! {
        <a
            class=move || {
                classnames(
                    &[
                        (
                            "py-3 font-semibold flex gap-5 items-center rounded-lg hover:bg-primary-700 transition-colors hover:text-white",
                            true,
                        ),
                        (
                            "text-white bg-gradient-to-l from-[#323337] to-[rgba(70,79,111,0.3)] shadow-[inset_0px_0.0625rem_0_rgba(255,255,255,0.05),0_0.25rem_0.5rem_0_rgba(0,0,0,0.1)]",
                            is_active(),
                        ),
                        ("text-primary-400", !is_active()),
                        ("px-0 justify-center", !expand()),
                        ("px-4", expand()),
                    ],
                )
            }
            href=href
        >
            {icon()}
            {move || if expand() { text } else { "" }}
        </a>
    }
}
