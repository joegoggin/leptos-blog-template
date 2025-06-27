use leptos::web_sys::window;
use leptos::{ev::MouseEvent, prelude::*};

use crate::components::icon::{moon::MoonIcon, sun::SunIcon};

#[component]
pub fn Nav() -> impl IntoView {
    let (is_dark_theme, set_is_dark_theme) = signal(false);

    fn set_theme(theme: &str) {
        if let Some(body) = window().and_then(|w| w.document()).and_then(|d| d.body()) {
            let _ = body.set_attribute("data-theme", theme);
        }
    }

    let on_icon_click = move |e: MouseEvent| {
        e.prevent_default();

        set_is_dark_theme.set(!is_dark_theme.get());
    };

    Effect::new(move || {
        #[cfg(feature = "hydrate")]
        {
            use crate::utils::local_storage::LocalStorageUtil;

            let local_storage_util = LocalStorageUtil::new();
            let theme = local_storage_util.get_item("theme");

            if let Some(theme) = theme {
                if theme == "dark" {
                    set_is_dark_theme.set(true);
                } else {
                    set_is_dark_theme.set(false);
                }
            }
        }
    });

    Effect::new(move || {
        let theme = if is_dark_theme.get() { "dark" } else { "light" };

        set_theme(theme);

        #[cfg(feature = "hydrate")]
        {
            use crate::utils::local_storage::LocalStorageUtil;

            let local_storage_util = LocalStorageUtil::new();

            local_storage_util.set_item("theme", theme);
        }
    });

    view! {
        <nav class="nav">
            <h4>"Leptos Blog Template"</h4>
            {move || {
                if is_dark_theme.get() {
                    view! {
                        <SunIcon color="white".to_string() is_button=true on:click=on_icon_click />
                    }
                        .into_any()
                } else {
                    view! {
                        <MoonIcon color="black".to_string() is_button=true on:click=on_icon_click />
                    }
                        .into_any()
                }
            }}
        </nav>
    }
}
