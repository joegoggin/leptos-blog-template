use leptos::prelude::*;

use crate::{
    api::post::get_posts,
    components::{
        loading::suspense::LoadingSuspense, message::error::ErrorMessage, post_preview::PostPreview,
    },
};

#[component]
pub fn HomePage() -> impl IntoView {
    let posts_resource = Resource::new(|| {}, |_| async move { get_posts().await });

    view! {
        <div class="home">
            <h1>"Posts"</h1>
            <LoadingSuspense>
                {move || {
                    posts_resource
                        .get()
                        .map(|result| match result {
                            Ok(posts) => {
                                posts
                                    .iter()
                                    .map(|post| {
                                        view! { <PostPreview post=post.clone() /> }
                                    })
                                    .collect::<Vec<_>>()
                                    .into_any()
                            }
                            Err(error) => {
                                view! { <ErrorMessage error=error.to_string() /> }.into_any()
                            }
                        })
                }}
            </LoadingSuspense>
        </div>
    }
}
