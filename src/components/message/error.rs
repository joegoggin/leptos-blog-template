use leptos::{children, prelude::*};

#[component]
pub fn ErrorMessage(error: String) -> impl IntoView {
    view! { <h5 class="error-message">{format!("Error: {}", error)}</h5> }
}
