use leptos::prelude::*;

use crate::api::post::get_posts;

#[component]
pub fn HomePage() -> impl IntoView {
    let hello_resource = LocalResource::new(|| async move { get_posts().await });

    view! {
        <div class="home">
            <h1>"Leptos Blog Template"</h1>
            <Suspense fallback=move || {
                view! { <p>"Loading..."</p> }
            }>
                {move || {
                    hello_resource
                        .get()
                        .map(|result| match result {
                            Ok(posts) => {
                                posts
                                    .iter()
                                    .map(|post| {
                                        view! { <p>{post.title.clone()}</p> }
                                    })
                                    .collect::<Vec<_>>()
                                    .into_any()
                            }
                            Err(e) => view! { <p>{format!("Error: {}", e)}</p> }.into_any(),
                        })
                }}
            </Suspense>
        </div>
    }
}
