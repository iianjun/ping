use crate::components::icons::{ChatIcon, GroupIcon, SearchIcon, SidebarIcon};
use crate::components::layout::ProfileFooter;
use crate::components::layout::TabLink;
use crate::utils::classnames;
use leptos::*;
#[component]
pub fn Sidebar() -> impl IntoView {
    let (expand, set_expand) = create_signal(true);
    let toggle = move |_| {
        set_expand(!expand());
    };
    view! {
        <nav class=move || {
            classnames(
                &[
                    ("w-[20rem]", expand()),
                    ("w-[6rem]", !expand()),
                    ("min-h-svh px-6 pb-6 float-left flex flex-col justify-between", true),
                ],
            )
        }>
            <div>
                <div class=move || {
                    classnames(
                        &[
                            ("flex items-center h-[7.5rem]", true),
                            ("justify-center", !expand()),
                            ("justify-between", expand()),
                        ],
                    )
                }>
                    <a
                        class=move || {
                            classnames(
                                &[
                                    ("text-3xl font-bold flex gap-3 items-center", true),
                                    ("hidden", !expand()),
                                ],
                            )
                        }
                        href="/"
                    >
                        <div class="ping-logo"></div>
                        Ping
                    </a>
                    <button on:click=toggle>
                        <SidebarIcon class="text-primary-500" />
                    </button>
                </div>
                <TabLink
                    href="/chat"
                    text="Chat"
                    icon=|| view! { <ChatIcon class="text-secondary-blue" /> }
                    expand=expand
                />
                <TabLink
                    href="/search"
                    text="Search"
                    icon=|| view! { <SearchIcon class="text-secondary-purple" /> }
                    expand=expand
                />
                <TabLink
                    href="/group"
                    text="Group"
                    icon=|| view! { <GroupIcon class="text-secondary-green" /> }
                    expand=expand
                />
            </div>
            <ProfileFooter expand=expand />
        </nav>
    }
}
