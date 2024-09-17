use crate::components::icons::LogoutIcon;
use crate::components::ping::Ping;
use crate::utils::classnames;
use leptos::*;
#[component]

pub fn ProfileFooter(expand: ReadSignal<bool>) -> impl IntoView {
    view! {
        <div class=move || classnames(
            &[
                ("py-[.625rem] flex items-center", true),
                ("flex-col gap-6 px-0", !expand()),
                ("px-[.625rem]", expand()),
            ],
        )>
            <div class="relative">
                <img
                    src="/assets/images/profile.png"
                    width="40"
                    height="40"
                    class="rounded-[50%]"
                />
                <Ping class="absolute right-0 bottom-0" />
            </div>
            {move || {
                if expand() {
                    view! {
                        <div class="ml-4">
                            <p class="text-sm font-semibold">Hasung Jun</p>
                            <p class="text-xs font-semibold text-primary-400">
                                hasungjunn@gmail.com
                            </p>
                        </div>
                    }
                        .into_view()
                } else {
                    view! { <></> }.into_view()
                }
            }}

            <div class="flex-1 flex justify-end">
                <button class="text-primary-400">
                    <LogoutIcon />
                </button>
            </div>
        </div>
    }
}
