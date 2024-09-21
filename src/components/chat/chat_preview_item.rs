use crate::models::User;
use leptos::*;
use leptos_router::*;
#[component]
pub fn ChatPreviewItem(
    #[prop(default = "lg")] size: &'static str,
    id: u32,
    user: User,
    last_message: String,
    last_message_at: Option<String>,
) -> impl IntoView {
    if size == "lg" {
        view! {
          <A
            href=format!("{}", id)
            class="flex gap-5 items-center p-3 w-full rounded-2xl transition-colors cursor-pointer hover:bg-primary-700"
          >
            <img
              src=user.profile
              alt="profile-picture"
              class="object-cover rounded-[.625rem]"
              style="width:76px; height:76px"
            />
            <div class="flex flex-col gap-2">
              <h2 class="text-lg font-semibold leading-6">{user.name}</h2>
              <p class="text-sm text-primary-400 line-clamp-2">{last_message}</p>
            </div>
          </A>
        }.into_view()
    } else {
        view! {
          <a
            href=format!("/chat/{}", id)
            class="flex gap-3 items-start px-2 transition-colors cursor-pointer py-[.625rem] rounded-[.625rem] w-[16.875rem] hover:bg-primary-700"
          >
            <img
              src=user.profile
              alt="profile-picture"
              class="object-cover rounded-[.625rem]"
              style="width:50px; height:50px"
            />
            <div class="flex flex-col gap-2">
              <h2 class="text-sm font-semibold">{user.name}</h2>
              <p class="text-xs text-primary-400 line-clamp-1">{last_message}</p>
            </div>
            <p class="text-xs font-semibold text-primary-400">{last_message_at}</p>
          </a>
        }.into_view()
    }
}
