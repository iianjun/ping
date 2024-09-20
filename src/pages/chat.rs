use crate::components::chat::ChatPreviewItem;
use leptos::*;
use leptos_meta::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatPreview {
    id: u32,
    last_message: String,
    name: String,
    profile: String,
}
#[server(GetChatPreviews, "/api", "GetJson")]
pub async fn get_previews() -> Result<Vec<ChatPreview>, ServerFnError> {
    let chat_previews = vec![
        ChatPreview {
            id: 1,
            name: "Nova Carter".to_string(),
            profile: "/assets/images/profile.png".to_string(),
            last_message: "Lorem ipsum odor amet, consectetuer adipiscing elit. Urna pellentesque at egestas est nunc ac vehicula orci. Class morbi ultricies vestibulum nullam posuere vehicula tempus enim. Donec consectetur pretium taciti maecenas lacus lacinia eget. Efficitur potenti ex eleifend condimentum taciti bibendum. Eleifend parturient non tempus habitasse habitasse aliquet lectus phasellus. Malesuada tellus fusce ligula penatibus, mauris mi nibh mus. Eleifend sodales consectetur vivamus dignissim at. Lacus ridiculus justo porta eu sit cursus natoque. Suscipit lectus facilisis augue vehicula dolor facilisi hendrerit. Aptent suspendisse metus et ante consequat. Luctus vel varius porta fermentum platea; justo imperdiet eros? Efficitur est venenatis pretium curabitur orci gravida maximus congue.".to_string(),
        },
        ChatPreview {
            id: 2,
            name: "Nova Carter".to_string(),
            profile: "/assets/images/profile-2.png".to_string(),
            last_message: "Lorem ipsum odor amet, consectetuer adipiscing elit. Urna pellentesque at egestas est nunc ac vehicula orci. Class morbi ultricies vestibulum nullam posuere vehicula tempus enim. Donec consectetur pretium taciti maecenas lacus lacinia eget. Efficitur potenti ex eleifend condimentum taciti bibendum. Eleifend parturient non tempus habitasse habitasse aliquet lectus phasellus. Malesuada tellus fusce ligula penatibus, mauris mi nibh mus. Eleifend sodales consectetur vivamus dignissim at. Lacus ridiculus justo porta eu sit cursus natoque. Suscipit lectus facilisis augue vehicula dolor facilisi hendrerit. Aptent suspendisse metus et ante consequat. Luctus vel varius porta fermentum platea; justo imperdiet eros? Efficitur est venenatis pretium curabitur orci gravida maximus congue.".to_string(),
        },
        ChatPreview {
            id: 3,
            name: "Ethan Blake".to_string(),
            profile: "/assets/images/profile-3.png".to_string(),
            last_message: "Lorem ipsum odor amet, consectetuer adipiscing elit. Urna pellentesque at egestas est nunc ac vehicula orci. Class morbi ultricies vestibulum nullam posuere vehicula tempus enim. Donec consectetur pretium taciti maecenas lacus lacinia eget. Efficitur potenti ex eleifend condimentum taciti bibendum. Eleifend parturient non tempus habitasse habitasse aliquet lectus phasellus. Malesuada tellus fusce ligula penatibus, mauris mi nibh mus. Eleifend sodales consectetur vivamus dignissim at. Lacus ridiculus justo porta eu sit cursus natoque. Suscipit lectus facilisis augue vehicula dolor facilisi hendrerit. Aptent suspendisse metus et ante consequat. Luctus vel varius porta fermentum platea; justo imperdiet eros? Efficitur est venenatis pretium curabitur orci gravida maximus congue.".to_string(),
        },
        ChatPreview {
            id: 4,
            name: "Lila Monroe".to_string(),
            profile: "/assets/images/profile-4.png".to_string(),
            last_message: "Lorem ipsum odor amet, consectetuer adipiscing elit. Urna pellentesque at egestas est nunc ac vehicula orci. Class morbi ultricies vestibulum nullam posuere vehicula tempus enim. Donec consectetur pretium taciti maecenas lacus lacinia eget. Efficitur potenti ex eleifend condimentum taciti bibendum. Eleifend parturient non tempus habitasse habitasse aliquet lectus phasellus. Malesuada tellus fusce ligula penatibus, mauris mi nibh mus. Eleifend sodales consectetur vivamus dignissim at. Lacus ridiculus justo porta eu sit cursus natoque. Suscipit lectus facilisis augue vehicula dolor facilisi hendrerit. Aptent suspendisse metus et ante consequat. Luctus vel varius porta fermentum platea; justo imperdiet eros? Efficitur est venenatis pretium curabitur orci gravida maximus congue.".to_string(),
        },
        ChatPreview {
            id: 5,
            name: "Aiden Rivers".to_string(),
            profile: "/assets/images/profile-5.png".to_string(),
            last_message: "Lorem ipsum odor amet, consectetuer adipiscing elit. Urna pellentesque at egestas est nunc ac vehicula orci. Class morbi ultricies vestibulum nullam posuere vehicula tempus enim. Donec consectetur pretium taciti maecenas lacus lacinia eget. Efficitur potenti ex eleifend condimentum taciti bibendum. Eleifend parturient non tempus habitasse habitasse aliquet lectus phasellus. Malesuada tellus fusce ligula penatibus, mauris mi nibh mus. Eleifend sodales consectetur vivamus dignissim at. Lacus ridiculus justo porta eu sit cursus natoque. Suscipit lectus facilisis augue vehicula dolor facilisi hendrerit. Aptent suspendisse metus et ante consequat. Luctus vel varius porta fermentum platea; justo imperdiet eros? Efficitur est venenatis pretium curabitur orci gravida maximus congue.".to_string(),
        },
        ChatPreview {
            id: 6,
            name: "Sophia Quinn".to_string(),
            profile: "/assets/images/profile-6.png".to_string(),
            last_message: "Lorem ipsum odor amet, consectetuer adipiscing elit. Urna pellentesque at egestas est nunc ac vehicula orci. Class morbi ultricies vestibulum nullam posuere vehicula tempus enim. Donec consectetur pretium taciti maecenas lacus lacinia eget. Efficitur potenti ex eleifend condimentum taciti bibendum. Eleifend parturient non tempus habitasse habitasse aliquet lectus phasellus. Malesuada tellus fusce ligula penatibus, mauris mi nibh mus. Eleifend sodales consectetur vivamus dignissim at. Lacus ridiculus justo porta eu sit cursus natoque. Suscipit lectus facilisis augue vehicula dolor facilisi hendrerit. Aptent suspendisse metus et ante consequat. Luctus vel varius porta fermentum platea; justo imperdiet eros? Efficitur est venenatis pretium curabitur orci gravida maximus congue.".to_string(),
        },
        ChatPreview {
            id: 7,
            name: "Mason Hale".to_string(),
            profile: "/assets/images/profile-7.png".to_string(),
            last_message: "Lorem ipsum odor amet, consectetuer adipiscing elit. Urna pellentesque at egestas est nunc ac vehicula orci. Class morbi ultricies vestibulum nullam posuere vehicula tempus enim. Donec consectetur pretium taciti maecenas lacus lacinia eget. Efficitur potenti ex eleifend condimentum taciti bibendum. Eleifend parturient non tempus habitasse habitasse aliquet lectus phasellus. Malesuada tellus fusce ligula penatibus, mauris mi nibh mus. Eleifend sodales consectetur vivamus dignissim at. Lacus ridiculus justo porta eu sit cursus natoque. Suscipit lectus facilisis augue vehicula dolor facilisi hendrerit. Aptent suspendisse metus et ante consequat. Luctus vel varius porta fermentum platea; justo imperdiet eros? Efficitur est venenatis pretium curabitur orci gravida maximus congue.".to_string(),
        },
        ChatPreview {
            id: 8,
            name: "Isla Harper".to_string(),
            profile: "/assets/images/profile-8.png".to_string(),
            last_message: "Lorem ipsum odor amet, consectetuer adipiscing elit. Urna pellentesque at egestas est nunc ac vehicula orci. Class morbi ultricies vestibulum nullam posuere vehicula tempus enim. Donec consectetur pretium taciti maecenas lacus lacinia eget. Efficitur potenti ex eleifend condimentum taciti bibendum. Eleifend parturient non tempus habitasse habitasse aliquet lectus phasellus. Malesuada tellus fusce ligula penatibus, mauris mi nibh mus. Eleifend sodales consectetur vivamus dignissim at. Lacus ridiculus justo porta eu sit cursus natoque. Suscipit lectus facilisis augue vehicula dolor facilisi hendrerit. Aptent suspendisse metus et ante consequat. Luctus vel varius porta fermentum platea; justo imperdiet eros? Efficitur est venenatis pretium curabitur orci gravida maximus congue.".to_string(),
        },
        ChatPreview {
            id: 9,
            name: "Liam Grey".to_string(),
            profile: "/assets/images/profile-9.png".to_string(),
            last_message: "Lorem ipsum odor amet, consectetuer adipiscing elit. Urna pellentesque at egestas est nunc ac vehicula orci. Class morbi ultricies vestibulum nullam posuere vehicula tempus enim. Donec consectetur pretium taciti maecenas lacus lacinia eget. Efficitur potenti ex eleifend condimentum taciti bibendum. Eleifend parturient non tempus habitasse habitasse aliquet lectus phasellus. Malesuada tellus fusce ligula penatibus, mauris mi nibh mus. Eleifend sodales consectetur vivamus dignissim at. Lacus ridiculus justo porta eu sit cursus natoque. Suscipit lectus facilisis augue vehicula dolor facilisi hendrerit. Aptent suspendisse metus et ante consequat. Luctus vel varius porta fermentum platea; justo imperdiet eros? Efficitur est venenatis pretium curabitur orci gravida maximus congue.".to_string(),
        },
        ChatPreview {
            id: 10,
            name: "Mia Turner".to_string(),
            profile: "/assets/images/profile-4.png".to_string(),
            last_message: "Lorem ipsum odor amet, consectetuer adipiscing elit. Urna pellentesque at egestas est nunc ac vehicula orci. Class morbi ultricies vestibulum nullam posuere vehicula tempus enim. Donec consectetur pretium taciti maecenas lacus lacinia eget. Efficitur potenti ex eleifend condimentum taciti bibendum. Eleifend parturient non tempus habitasse habitasse aliquet lectus phasellus. Malesuada tellus fusce ligula penatibus, mauris mi nibh mus. Eleifend sodales consectetur vivamus dignissim at. Lacus ridiculus justo porta eu sit cursus natoque. Suscipit lectus facilisis augue vehicula dolor facilisi hendrerit. Aptent suspendisse metus et ante consequat. Luctus vel varius porta fermentum platea; justo imperdiet eros? Efficitur est venenatis pretium curabitur orci gravida maximus congue.".to_string(),
        },
        ChatPreview {
            id: 11,
            name: "Lucas Brooks".to_string(),
            profile: "/assets/images/profile.png".to_string(),
            last_message: "Lorem ipsum odor amet, consectetuer adipiscing elit. Urna pellentesque at egestas est nunc ac vehicula orci. Class morbi ultricies vestibulum nullam posuere vehicula tempus enim. Donec consectetur pretium taciti maecenas lacus lacinia eget. Efficitur potenti ex eleifend condimentum taciti bibendum. Eleifend parturient non tempus habitasse habitasse aliquet lectus phasellus. Malesuada tellus fusce ligula penatibus, mauris mi nibh mus. Eleifend sodales consectetur vivamus dignissim at. Lacus ridiculus justo porta eu sit cursus natoque. Suscipit lectus facilisis augue vehicula dolor facilisi hendrerit. Aptent suspendisse metus et ante consequat. Luctus vel varius porta fermentum platea; justo imperdiet eros? Efficitur est venenatis pretium curabitur orci gravida maximus congue.".to_string(),
        },
    ];
    Ok(chat_previews)
}
#[component]
pub fn ChatPage() -> impl IntoView {
    let data = create_resource(|| (), |_| async move { get_previews().await });

    view! {
        <Title text="Chat" />
        <section class="w-full h-full flex flex-col rounded-3xl bg-primary-800">
            <div class="py-3 px-6 border-b border-primary-700">
                <h1 class="text-2xl font-semibold leading-10">My Chats</h1>
            </div>
            <div class="py-4 px-6 flex flex-col overflow-auto flex-1">
                <Suspense fallback=move || {
                    view! { <p>Loading...</p> }
                }>
                    <ErrorBoundary fallback=|_| {
                        view! { <p>"Something went wrong."</p> }
                    }>
                        {move || {
                            data.get()
                                .map(|x| {
                                    x.map(|result| {
                                        view! {
                                            <For
                                                each=move || result.clone().into_iter().enumerate()
                                                key=|(i, _)| *i
                                                children=|(_, preview)| {
                                                    view! {
                                                        <ChatPreviewItem
                                                            id=preview.id
                                                            profile=preview.profile
                                                            name=preview.name
                                                            last_message=preview.last_message
                                                        />
                                                    }
                                                }
                                            />
                                        }
                                    })
                                })
                        }}
                    </ErrorBoundary>
                </Suspense>
            </div>
        </section>
    }
}
