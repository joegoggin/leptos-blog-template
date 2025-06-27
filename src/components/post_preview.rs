use leptos::prelude::*;

use crate::models::post::Post;

#[component]
pub fn PostPreview(post: Post) -> impl IntoView {
    let first_paragraph = post.get_paragraphs().clone();

    view! {
        <div class="post-preview">
            <h3>{post.title.clone()}</h3>
            <h5>{post.get_date()}</h5>
            <p>{first_paragraph.len()}</p>
            <button>Read More</button>
        </div>
    }
}
