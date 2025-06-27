use leptos::prelude::*;

use crate::components::loading::spinner::LoadingSpinner;

#[component]
pub fn LoadingSuspense(children: Children) -> impl IntoView {
    view! {
        <Suspense fallback=move || {
            view! { <LoadingSpinner /> }
        }>{children()}</Suspense>
    }
}
