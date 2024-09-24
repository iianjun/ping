#![allow(unused_imports)]
use crate::models::{ChatRoomMember, User};
use leptos::*;
use leptos_router::*;

#[server(GetChatRoomMember, "/api", "GetJson")]
pub async fn get_chat_user(room_id: String) -> Result<ChatRoomMember, ServerFnError> {
    let room_member = ChatRoomMember {
        id: 1,
        user: User {
            id: 1,
            name: "Nova Carter".to_string(),
            profile: "/assets/images/profile.png".to_string(),
        },
        chat_room_id: 1,
        is_online: true,
    };
    Ok(room_member)
}
#[component]
pub fn ChatHeader() -> impl IntoView {
    let params = use_params_map();
    let data = create_resource(
        move || params.get().get("id").cloned().unwrap_or_default(),
        |chat_room_id| async { get_chat_user(chat_room_id).await },
    );
    view! {
      <div class="flex items-center p-2 gap-[.625rem]">
        <Suspense fallback=move || view! { <p>"Loading..."</p> }>
          <ErrorBoundary fallback=|_| {
            view! { <p>"Something went wrong, please try again later."</p> }
          }>
            {move || {
              data
                .get()
                .map(move |x| {
                  x.map(move |result| {
                    view! {
                      <img
                        src=result.user.profile
                        alt="profile-picture"
                        class="object-cover rounded-[50%]"
                        style="width:50px; height:50px"
                      />
                      <div class="flex flex-col">
                        <h1 class="text-xl font-semibold">{result.user.name}</h1>
                        <p class="text-sm text-primary-200">
                          {move || if result.is_online { "Online" } else { "Offline" }}
                        </p>
                      </div>
                    }
                  })
                })
            }}
          </ErrorBoundary>
        </Suspense>
      // alt="profile-picture"
      // class="object-cover rounded-[.625rem]"
      // style="width:76px; height:76px"
      // />
      </div>
    }
}
