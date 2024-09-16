use leptos::{component, view, IntoView};
#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <nav class="w-[20rem] min-h-svh px-6 pb-6 float-left">
            <div class="flex justify-between items-center h-[7.5rem]">
                <div class="flex gap-3 items-center">
                    <div class="ping-logo"></div>
                        <a class="text-3xl font-bold" href="/">Ping</a>
                </div>
            </div>
        </nav>
    }
}
