use leptos::prelude::*;

#[component]
pub fn MoonIcon(color: String, #[prop(default = false)] is_button: bool) -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke=color
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            role=if is_button { Some("button") } else { None }
        >
            <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
        </svg>
    }
}
