use leptos::{ev::MouseEvent, logging::log, prelude::*};
use leptos_router::hooks::use_navigate;

use crate::models::post::Post;

#[component]
pub fn PostPreview(post: Post) -> impl IntoView {
    let first_paragraph = post.get_paragraphs()[0].clone();

    if &post.title == "Most Recent" {
        log!("{:#?}", post.get_paragraphs());
    }

    let navigate = use_navigate();

    let on_click = move |e: MouseEvent| {
        e.prevent_default();

        navigate(
            &format!("/post/{}", post.id.to_string()),
            Default::default(),
        )
    };

    view! {
        <div class="post-preview">
            <h3>{post.title.clone()}</h3>
            <h5>{post.get_date()}</h5>
            <p>{first_paragraph}</p>
            <button on:click=on_click>Read More</button>
        </div>
    }
}
