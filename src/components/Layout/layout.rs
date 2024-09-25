use crate::components::layout::Sidebar;
use crate::pages::auth::Auth;
use crate::pages::chat::ChatPage;
use crate::pages::chat_detail::ChatDetailPage;
use crate::pages::home::HomePage;
use crate::utils::classnames;
use leptos::*;
use leptos_router::*;
#[component]
pub fn Layout() -> impl IntoView {
    let expand = expect_context::<RwSignal<bool>>().read_only();
    let location = use_location();
    let pathname = move || location.pathname.get();
    let is_auth = pathname() == "/auth";
    view! {
      {move || {
        if is_auth { view! { <></> }.into_view() } else { view! { <Sidebar /> }.into_view() }
      }}

      <main class=move || {
        classnames(
          &[
            ("h-svh overflow-auto py-6 pr-6", true),
            ("ml-[20rem]", expand() && !is_auth),
            ("ml-[6rem]", !expand() && !is_auth),
            ("flex justify-center items-center", is_auth),
          ],
        )
      }>
        <Routes>
          <Route path="" view=HomePage />
          <Route path="/auth" view=Auth />
          <Route path="/chat" view=ChatPage />
          <Route path="/chat/:id" view=ChatDetailPage />
        </Routes>
      </main>
    }
}
