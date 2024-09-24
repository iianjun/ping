use crate::models::Message;
use leptos::*;
#[component]
pub fn MessageItem(message: Message) -> impl IntoView {
    view! {
      <div class="flex gap-2 items-start px-2">
        <img
          class="w-[3.125rem] h-[3.125rem] rounded-[50%]"
          src=message.user.profile
          alt="user-profile"
        />
        <div class="flex flex-col gap-2">
          <div class="flex gap-2 items-center">
            <p class="text-sm font-semibold">{message.user.name}</p>
            <p class="text-xs font-semibold text-primary-200">{message.sent_at}</p>
          </div>
          <p class="text-sm">{message.content}</p>
        </div>
      </div>
    }
}
