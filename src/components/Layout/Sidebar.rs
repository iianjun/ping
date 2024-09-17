use crate::components::icons::{ChatIcon, GroupIcon, SearchIcon, SidebarIcon};
use crate::components::layout::ProfileFooter;
use crate::components::layout::TabLink;
use leptos::{component, view, IntoView};
#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <nav class="w-[20rem] min-h-svh px-6 pb-6 float-left flex flex-col justify-between">
            <div>
                <div class="flex justify-between items-center h-[7.5rem]">
                    <div class="flex gap-3 items-center">
                        <div class="ping-logo"></div>
                        <a class="text-3xl font-bold" href="/">
                            Ping
                        </a>
                    </div>
                    <button>
                        <SidebarIcon class="text-primary-500" />
                    </button>
                </div>
                <TabLink
                    href="/chat"
                    text="Chat"
                    icon=|| view! { <ChatIcon class="text-secondary-blue" /> }
                />
                <TabLink
                    href="/search"
                    text="Search"
                    icon=|| view! { <SearchIcon class="text-secondary-purple" /> }
                />
                <TabLink
                    href="/group"
                    text="Group"
                    icon=|| view! { <GroupIcon class="text-secondary-green" /> }
                />
            </div>
            <ProfileFooter />
        </nav>
    }
}
