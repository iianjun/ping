use crate::components::chat::ChatPreviewItem;
use leptos::*;
#[derive(Clone)]
struct ChatPreview {
    id: u32,
    last_message: &'static str,
    name: &'static str,
    profile: &'static str,
}
#[component]
pub fn ChatPage() -> impl IntoView {
    let chat_previews = vec![
        ChatPreview {
            id: 1,
            name: "Nova Carter",
            profile: "/assets/images/profile.png",
            last_message: "Lorem ipsum odor amet, consectetuer adipiscing elit. Urna pellentesque at egestas est nunc ac vehicula orci. Class morbi ultricies vestibulum nullam posuere vehicula tempus enim. Donec consectetur pretium taciti maecenas lacus lacinia eget. Efficitur potenti ex eleifend condimentum taciti bibendum. Eleifend parturient non tempus habitasse habitasse aliquet lectus phasellus. Malesuada tellus fusce ligula penatibus, mauris mi nibh mus. Eleifend sodales consectetur vivamus dignissim at. Lacus ridiculus justo porta eu sit cursus natoque. Suscipit lectus facilisis augue vehicula dolor facilisi hendrerit. Aptent suspendisse metus et ante consequat. Luctus vel varius porta fermentum platea; justo imperdiet eros? Efficitur est venenatis pretium curabitur orci gravida maximus congue.",
        },
        ChatPreview {
            id: 2,
            name: "Nova Carter",
            profile: "/assets/images/profile-2.png",
            last_message: "Lorem ipsum odor amet, consectetuer adipiscing elit. Urna pellentesque at egestas est nunc ac vehicula orci. Class morbi ultricies vestibulum nullam posuere vehicula tempus enim. Donec consectetur pretium taciti maecenas lacus lacinia eget. Efficitur potenti ex eleifend condimentum taciti bibendum. Eleifend parturient non tempus habitasse habitasse aliquet lectus phasellus. Malesuada tellus fusce ligula penatibus, mauris mi nibh mus. Eleifend sodales consectetur vivamus dignissim at. Lacus ridiculus justo porta eu sit cursus natoque. Suscipit lectus facilisis augue vehicula dolor facilisi hendrerit. Aptent suspendisse metus et ante consequat. Luctus vel varius porta fermentum platea; justo imperdiet eros? Efficitur est venenatis pretium curabitur orci gravida maximus congue.",
        },
        ChatPreview {
            id: 3,
            name: "Ethan Blake",
            profile: "/assets/images/profile-3.png",
            last_message: "Lorem ipsum odor amet, consectetuer adipiscing elit. Urna pellentesque at egestas est nunc ac vehicula orci. Class morbi ultricies vestibulum nullam posuere vehicula tempus enim. Donec consectetur pretium taciti maecenas lacus lacinia eget. Efficitur potenti ex eleifend condimentum taciti bibendum. Eleifend parturient non tempus habitasse habitasse aliquet lectus phasellus. Malesuada tellus fusce ligula penatibus, mauris mi nibh mus. Eleifend sodales consectetur vivamus dignissim at. Lacus ridiculus justo porta eu sit cursus natoque. Suscipit lectus facilisis augue vehicula dolor facilisi hendrerit. Aptent suspendisse metus et ante consequat. Luctus vel varius porta fermentum platea; justo imperdiet eros? Efficitur est venenatis pretium curabitur orci gravida maximus congue.",
        },
        ChatPreview {
            id: 4,
            name: "Lila Monroe",
            profile: "/assets/images/profile-4.png",
            last_message: "Lorem ipsum odor amet, consectetuer adipiscing elit. Urna pellentesque at egestas est nunc ac vehicula orci. Class morbi ultricies vestibulum nullam posuere vehicula tempus enim. Donec consectetur pretium taciti maecenas lacus lacinia eget. Efficitur potenti ex eleifend condimentum taciti bibendum. Eleifend parturient non tempus habitasse habitasse aliquet lectus phasellus. Malesuada tellus fusce ligula penatibus, mauris mi nibh mus. Eleifend sodales consectetur vivamus dignissim at. Lacus ridiculus justo porta eu sit cursus natoque. Suscipit lectus facilisis augue vehicula dolor facilisi hendrerit. Aptent suspendisse metus et ante consequat. Luctus vel varius porta fermentum platea; justo imperdiet eros? Efficitur est venenatis pretium curabitur orci gravida maximus congue.",
        },
        ChatPreview {
            id: 5,
            name: "Aiden Rivers",
            profile: "/assets/images/profile-5.png",
            last_message: "Lorem ipsum odor amet, consectetuer adipiscing elit. Urna pellentesque at egestas est nunc ac vehicula orci. Class morbi ultricies vestibulum nullam posuere vehicula tempus enim. Donec consectetur pretium taciti maecenas lacus lacinia eget. Efficitur potenti ex eleifend condimentum taciti bibendum. Eleifend parturient non tempus habitasse habitasse aliquet lectus phasellus. Malesuada tellus fusce ligula penatibus, mauris mi nibh mus. Eleifend sodales consectetur vivamus dignissim at. Lacus ridiculus justo porta eu sit cursus natoque. Suscipit lectus facilisis augue vehicula dolor facilisi hendrerit. Aptent suspendisse metus et ante consequat. Luctus vel varius porta fermentum platea; justo imperdiet eros? Efficitur est venenatis pretium curabitur orci gravida maximus congue.",
        },
        ChatPreview {
            id: 6,
            name: "Sophia Quinn",
            profile: "/assets/images/profile-6.png",
            last_message: "Lorem ipsum odor amet, consectetuer adipiscing elit. Urna pellentesque at egestas est nunc ac vehicula orci. Class morbi ultricies vestibulum nullam posuere vehicula tempus enim. Donec consectetur pretium taciti maecenas lacus lacinia eget. Efficitur potenti ex eleifend condimentum taciti bibendum. Eleifend parturient non tempus habitasse habitasse aliquet lectus phasellus. Malesuada tellus fusce ligula penatibus, mauris mi nibh mus. Eleifend sodales consectetur vivamus dignissim at. Lacus ridiculus justo porta eu sit cursus natoque. Suscipit lectus facilisis augue vehicula dolor facilisi hendrerit. Aptent suspendisse metus et ante consequat. Luctus vel varius porta fermentum platea; justo imperdiet eros? Efficitur est venenatis pretium curabitur orci gravida maximus congue.",
        },
        ChatPreview {
            id: 7,
            name: "Mason Hale",
            profile: "/assets/images/profile-7.png",
            last_message: "Lorem ipsum odor amet, consectetuer adipiscing elit. Urna pellentesque at egestas est nunc ac vehicula orci. Class morbi ultricies vestibulum nullam posuere vehicula tempus enim. Donec consectetur pretium taciti maecenas lacus lacinia eget. Efficitur potenti ex eleifend condimentum taciti bibendum. Eleifend parturient non tempus habitasse habitasse aliquet lectus phasellus. Malesuada tellus fusce ligula penatibus, mauris mi nibh mus. Eleifend sodales consectetur vivamus dignissim at. Lacus ridiculus justo porta eu sit cursus natoque. Suscipit lectus facilisis augue vehicula dolor facilisi hendrerit. Aptent suspendisse metus et ante consequat. Luctus vel varius porta fermentum platea; justo imperdiet eros? Efficitur est venenatis pretium curabitur orci gravida maximus congue.",
        },
        ChatPreview {
            id: 8,
            name: "Isla Harper",
            profile: "/assets/images/profile-8.png",
            last_message: "Lorem ipsum odor amet, consectetuer adipiscing elit. Urna pellentesque at egestas est nunc ac vehicula orci. Class morbi ultricies vestibulum nullam posuere vehicula tempus enim. Donec consectetur pretium taciti maecenas lacus lacinia eget. Efficitur potenti ex eleifend condimentum taciti bibendum. Eleifend parturient non tempus habitasse habitasse aliquet lectus phasellus. Malesuada tellus fusce ligula penatibus, mauris mi nibh mus. Eleifend sodales consectetur vivamus dignissim at. Lacus ridiculus justo porta eu sit cursus natoque. Suscipit lectus facilisis augue vehicula dolor facilisi hendrerit. Aptent suspendisse metus et ante consequat. Luctus vel varius porta fermentum platea; justo imperdiet eros? Efficitur est venenatis pretium curabitur orci gravida maximus congue.",
        },
        ChatPreview {
            id: 9,
            name: "Liam Grey",
            profile: "/assets/images/profile-9.png",
            last_message: "Lorem ipsum odor amet, consectetuer adipiscing elit. Urna pellentesque at egestas est nunc ac vehicula orci. Class morbi ultricies vestibulum nullam posuere vehicula tempus enim. Donec consectetur pretium taciti maecenas lacus lacinia eget. Efficitur potenti ex eleifend condimentum taciti bibendum. Eleifend parturient non tempus habitasse habitasse aliquet lectus phasellus. Malesuada tellus fusce ligula penatibus, mauris mi nibh mus. Eleifend sodales consectetur vivamus dignissim at. Lacus ridiculus justo porta eu sit cursus natoque. Suscipit lectus facilisis augue vehicula dolor facilisi hendrerit. Aptent suspendisse metus et ante consequat. Luctus vel varius porta fermentum platea; justo imperdiet eros? Efficitur est venenatis pretium curabitur orci gravida maximus congue.",
        },
        ChatPreview {
            id: 10,
            name: "Mia Turner",
            profile: "/assets/images/profile-4.png",
            last_message: "Lorem ipsum odor amet, consectetuer adipiscing elit. Urna pellentesque at egestas est nunc ac vehicula orci. Class morbi ultricies vestibulum nullam posuere vehicula tempus enim. Donec consectetur pretium taciti maecenas lacus lacinia eget. Efficitur potenti ex eleifend condimentum taciti bibendum. Eleifend parturient non tempus habitasse habitasse aliquet lectus phasellus. Malesuada tellus fusce ligula penatibus, mauris mi nibh mus. Eleifend sodales consectetur vivamus dignissim at. Lacus ridiculus justo porta eu sit cursus natoque. Suscipit lectus facilisis augue vehicula dolor facilisi hendrerit. Aptent suspendisse metus et ante consequat. Luctus vel varius porta fermentum platea; justo imperdiet eros? Efficitur est venenatis pretium curabitur orci gravida maximus congue.",
        },
        ChatPreview {
            id: 11,
            name: "Lucas Brooks",
            profile: "/assets/images/profile.png",
            last_message: "Lorem ipsum odor amet, consectetuer adipiscing elit. Urna pellentesque at egestas est nunc ac vehicula orci. Class morbi ultricies vestibulum nullam posuere vehicula tempus enim. Donec consectetur pretium taciti maecenas lacus lacinia eget. Efficitur potenti ex eleifend condimentum taciti bibendum. Eleifend parturient non tempus habitasse habitasse aliquet lectus phasellus. Malesuada tellus fusce ligula penatibus, mauris mi nibh mus. Eleifend sodales consectetur vivamus dignissim at. Lacus ridiculus justo porta eu sit cursus natoque. Suscipit lectus facilisis augue vehicula dolor facilisi hendrerit. Aptent suspendisse metus et ante consequat. Luctus vel varius porta fermentum platea; justo imperdiet eros? Efficitur est venenatis pretium curabitur orci gravida maximus congue.",
        },
    ];

    view! {
        <section class="w-full h-full flex flex-col rounded-3xl bg-primary-800">
            <div class="py-3 px-6 border-b border-primary-700">
                <h1 class="text-2xl font-semibold leading-10">My Chats</h1>
            </div>
            <div id="wrap" class="py-4 px-6 flex flex-col overflow-auto flex-1">
                {chat_previews
                    .iter()
                    .map(|preview| {
                        view! {
                            <ChatPreviewItem
                                id=preview.id
                                profile=preview.profile
                                name=preview.name
                                last_message=preview.last_message
                            />
                        }
                    })
                    .collect_view()}
            </div>
        </section>
    }
}
