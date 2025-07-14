use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::{
    components::nav::Nav,
    pages::{home::HomePage, post::PostPage},
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-blog-template.css" />
        <Title text="Welcome to Leptos" />
        <Router>
            <Nav />
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=path!("/") view=HomePage />
                    <Route path=path!("/post/:post_id") view=PostPage />
                </Routes>
            </main>
        </Router>
    }
}
