use crate::components::icons::LogoutIcon;
use crate::components::ping::Ping;
use leptos::*;
#[component]
pub fn ProfileFooter() -> impl IntoView {
    view! {
        <div class="p-[.625rem] flex justify-between items-center">
            <div class="flex gap-4 items-center">
                <div class="relative">
                    <img
                        src="/assets/images/profile.png"
                        width="40"
                        height="40"
                        class="rounded-[50%]"
                    />
                    <Ping class="absolute right-0 bottom-0" />
                </div>
                <div>
                    <p class="text-sm font-semibold">Hasung Jun</p>
                    <p class="text-xs font-semibold text-primary-400">hasungjunn@gmail.com</p>
                </div>
            </div>
            <button class="text-primary-400">
                <LogoutIcon />
            </button>
        </div>
    }
}
