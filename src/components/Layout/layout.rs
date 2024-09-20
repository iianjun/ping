use crate::components::layout::Sidebar;
use crate::pages::chat::ChatPage;
use crate::pages::chat_detail::ChatDetailPage;
use crate::pages::home::HomePage;
use crate::utils::classnames;
use leptos::*;
use leptos_router::*;
#[component]
pub fn Layout() -> impl IntoView {
    let expand = expect_context::<RwSignal<bool>>().read_only();
    view! {
        <Sidebar />
        <main class=move || {
            classnames(
                &[
                    ("h-svh overflow-auto py-6 pr-6", true),
                    ("ml-[20rem]", expand()),
                    ("ml-[6rem]", !expand()),
                ],
            )
        }>
            <Routes>
                <Route path="" view=HomePage />
                <Route path="/chat" view=ChatPage />
                <Route path="/chat/:id" view=ChatDetailPage />
            </Routes>
        </main>
    }
}
