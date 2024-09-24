use crate::components::chat::MessageItem;
use crate::models::{Message, User};
use leptos::*;
use leptos_router::*;

#[server(GetChatMessages, "/api", "GetJson")]
pub async fn get_messages(room_id: String) -> Result<Vec<Message>, ServerFnError> {
    let messages = vec![
        Message {
            id: 1,
            chat_room_id: 1,
            user: User {
                id: 1,
                name: "Nova Carter".to_string(),
                profile: "/assets/images/profile.png".to_string(),
            },
            content: "Lorem ipsum odor amet, consectetuer adipiscing elit.".to_string(),
            sent_at: Some("3:48PM".to_string()),
        },
        Message {
            id: 1,
            chat_room_id: 1,
            user: User {
                id: 1,
                name: "Nova Carter".to_string(),
                profile: "/assets/images/profile.png".to_string(),
            },
            content: "Lorem ipsum odor amet, consectetuer adipiscing elit.".to_string(),
            sent_at: Some("3:48PM".to_string()),
        },
        Message {
            id: 1,
            chat_room_id: 1,
            user: User {
                id: 1,
                name: "Nova Carter".to_string(),
                profile: "/assets/images/profile.png".to_string(),
            },
            content: "Lorem ipsum odor amet, consectetuer adipiscing elit.".to_string(),
            sent_at: Some("3:48PM".to_string()),
        },
        Message {
            id: 1,
            chat_room_id: 1,
            user: User {
                id: 1,
                name: "Nova Carter".to_string(),
                profile: "/assets/images/profile.png".to_string(),
            },
            content: "Lorem ipsum odor amet, consectetuer adipiscing elit.".to_string(),
            sent_at: Some("3:48PM".to_string()),
        },
        Message {
            id: 1,
            chat_room_id: 1,
            user: User {
                id: 1,
                name: "Nova Carter".to_string(),
                profile: "/assets/images/profile.png".to_string(),
            },
            content: "Lorem ipsum odor amet, consectetuer adipiscing elit.".to_string(),
            sent_at: Some("3:48PM".to_string()),
        },
        Message {
            id: 1,
            chat_room_id: 1,
            user: User {
                id: 1,
                name: "Nova Carter".to_string(),
                profile: "/assets/images/profile.png".to_string(),
            },
            content: "Lorem ipsum odor amet, consectetuer adipiscing elit.".to_string(),
            sent_at: Some("3:48PM".to_string()),
        },
        Message {
            id: 1,
            chat_room_id: 1,
            user: User {
                id: 1,
                name: "Nova Carter".to_string(),
                profile: "/assets/images/profile.png".to_string(),
            },
            content: "Lorem ipsum odor amet, consectetuer adipiscing elit.".to_string(),
            sent_at: Some("3:48PM".to_string()),
        },
    ];
    Ok(messages)
}
#[component]
pub fn Messages() -> impl IntoView {
    let params = use_params_map();
    let data = create_resource(
        move || params.get().get("id").cloned().unwrap_or_default(),
        |chat_room_id| async { get_messages(chat_room_id).await },
    );
    view! {
      <div class="flex overflow-auto flex-col flex-1 gap-2 justify-end h-full">

        <Suspense fallback=move || view! { <p>Loading...</p> }>
          <ErrorBoundary fallback=|_| {
            view! { <p>Something went wrong!</p> }
          }>
            {move || {
              data
                .get()
                .map(|x| {
                  x.map(|result| {
                    view! {
                      <For
                        each=move || result.clone().into_iter().enumerate()
                        key=|(i, _)| *i
                        children=|(_, message)| {
                          view! { <MessageItem message=message /> }
                        }
                      />
                    }
                  })
                })
            }}
          </ErrorBoundary>
        </Suspense>
      </div>
    }
}
