use leptos::*;

#[component]
pub fn TabLink<F, IV>(href: &'static str, text: &'static str, icon: F) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
        <a
            class="py-3 px-4 font-semibold text-primary-400 flex gap-5 items-center rounded-lg hover:bg-primary-700 transition-colors hover:text-white"
            href=href
        >
            {icon()}
            {text}
        </a>
    }
}
