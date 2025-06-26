#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use leptos_blog_template::{
        app::*,
        core::{context::Context, dotenv::DotEnv},
        layout::layout,
    };
    use sqlx::PgPool;

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);
    let env = DotEnv::new();
    let database_url = env.database_url.clone();
    let db = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database.");

    let ctx = Context { env, db };

    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            move || provide_context(ctx.clone()),
            {
                let leptos_options = leptos_options.clone();
                move || layout(leptos_options.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler(layout))
        .with_state(leptos_options);

    log!("listening on http://{}", &addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
