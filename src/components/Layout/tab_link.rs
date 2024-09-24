use crate::utils::classnames;
use leptos::*;
use leptos_router::use_location;
#[component]
pub fn TabLink<F, IV>(href: &'static str, text: &'static str, icon: F) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    let expand = expect_context::<RwSignal<bool>>().read_only();
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
              ("text-white bg-selected-gradient shadow-selected", is_active()),
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
