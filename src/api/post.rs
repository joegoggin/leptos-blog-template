use leptos::{prelude::*, server_fn::codec::GetUrl};

use crate::models::post::Post;

#[server(GetPosts, endpoint = "get_posts", input = GetUrl)]
pub async fn get_posts() -> Result<Vec<Post>, ServerFnError> {
    use crate::core::context::Context;
    use sqlx::query_as;

    let ctx = expect_context::<Context>();

    let posts = query_as!(
        Post,
        r#"
        SELECT * 
        FROM post
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(&ctx.db)
    .await?;

    Ok(posts)
}
