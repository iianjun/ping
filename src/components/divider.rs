use leptos::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Vertical,
    Horizontal,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Vertical
    }
}

#[component]
pub fn Divider(#[prop(optional)] dir: Direction) -> impl IntoView {
    let class = match dir {
        Direction::Vertical => "clear-both flex h-full max-h-full border-l border-primary-700",
        Direction::Horizontal => "clear-both flex w-full max-w-full border-t border-primary-700",
    };
    view! { <div class=class></div> }
}
