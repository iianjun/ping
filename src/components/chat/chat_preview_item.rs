use leptos::*;
use leptos_router::*;
#[component]
pub fn ChatPreviewItem(
    #[prop(default = "lg")] size: &'static str,
    id: u32,
    profile: String,
    name: String,
    last_message: String,
) -> impl IntoView {
    if size == "lg" {
        view! {
            <A
                href=format!("{}", id)
                class="cursor-pointer flex gap-5 items-center p-3 w-full rounded-2xl transition-colors hover:bg-primary-700"
            >
                <img
                    src=profile
                    alt="profile-picture"
                    class="rounded-[.625rem] object-cover"
                    style="width:76px; height:76px"
                />
                <div class="flex flex-col gap-2">
                    <h2 class="text-lg leading-6 font-semibold">{name}</h2>
                    <p class="text-sm text-primary-400 line-clamp-2">{last_message}</p>
                </div>
            </A>
        }.into_view()
    } else {
        view! { <div></div> }.into_view()
    }
}
