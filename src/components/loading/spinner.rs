use leptos::prelude::*;

#[component]
pub fn LoadingSpinner() -> impl IntoView {
    view! {
        <div class="loading-spinner">
            <h4>Loading</h4>
            <div class="loading-spinner__spinner"></div>
        </div>
    }
}
