use crate::components::chat::ChatPreviewItem;
use crate::components::divider::Divider;
use crate::pages::chat::get_previews;
use leptos::*;
#[component]
pub fn ChatDetailPage() -> impl IntoView {
    let data = create_resource(|| (), |_| async move { get_previews().await });
    view! {
      <section class="flex flex-col w-full h-full rounded-3xl bg-primary-800">
        <div class="flex flex-1 py-4 h-full">
          <div class="flex overflow-auto flex-col px-3">
            <Suspense fallback=move || {
              view! { <p>Loading...</p> }
            }>
              <ErrorBoundary fallback=|_| {
                view! { <p>"Something went wrong."</p> }
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
                            children=|(_, preview)| {
                              view! {
                                <ChatPreviewItem
                                  id=preview.id
                                  user=preview.user
                                  last_message=preview.last_message
                                  last_message_at=preview.last_message_at
                                  size="sm"
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
          <Divider />
          <div class="flex-1 px-3">Content</div>
        </div>
      </section>
    }
}
