use leptos::prelude::*;
use leptos_router::{hooks::use_params, params::Params};

#[derive(Params, PartialEq, Debug)]
struct PostParams {
    post_id: Option<String>,
}

#[component]
pub fn PostPage() -> impl IntoView {
    let params = use_params::<PostParams>();
    let (post_id, set_post_id) = signal::<Option<String>>(None);

    Effect::new(move || match params.read().as_ref().ok() {
        Some(params) => {
            set_post_id.set(params.post_id.clone());
        }
        None => {
            set_post_id.set(None);
        }
    });

    view! {
        <div class="post-page">
            <h1>Post Page</h1>
            <h2>{move || post_id.get()}</h2>
        </div>
    }
}
