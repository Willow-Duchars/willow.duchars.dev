// Modules
mod desktop;
mod taskbar;
mod window;
// Re-exports
pub use desktop::Desktop;
pub use taskbar::Taskbar;
pub use window::Window;
// Imports
use crate::{constants::icons, core::sprite_data::SpriteData};
use leptos::{ev, prelude::*};
use leptos_maybe_callback::MaybeCallback;

/// Creates a square icon with the provided icon src and optional alt text
#[component]
pub fn icon(
    src: &'static str,
    #[prop(optional)] alt: &'static str,
    #[prop(optional)] inverted: bool,
) -> impl IntoView {
    if !inverted {
        view! { <img class="icon" src=src alt=alt/> }.into_any()
    } else {
        view! { <img class="inverted icon" src=src alt=alt/> }.into_any()
    }
}

/// Creates a semantic button with required aria label and optional on click closures
#[component]
pub fn button(
    icon: &'static str,
    #[prop(into)] label: String,
    #[prop(optional)] title: Option<&'static str>,
    #[prop(optional)] link: Option<&'static str>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] inverted_icon: bool,
    #[prop(into, optional)] on_click: MaybeCallback<ev::MouseEvent>,
    #[prop(into, optional)] on_dblclick: MaybeCallback<ev::MouseEvent>,
) -> impl IntoView {
    let title = match title {
        Some(title) => view! { <h1>{title}</h1> }.into_any(),
        None => ().into_any(),
    };
    match link {
        Some(link) => {
            let use_link = |_| {
                let _ = window().open_with_url_and_target(link, "_blank");
            };
            view! {
                <button type="button" aria_label=label disabled=disabled on:dblclick=use_link>
                    <a href=link target="_blank" rel="external" on:click=|e| e.prevent_default()>
                        <Icon src=icon inverted=inverted_icon/>
                        <Icon src=icons::EXTERNAL/>
                        {title}
                    </a>
                </button>
            }
            .into_any()
        }
        None => {
            let on_click = on_click.into_handler();
            let on_dblclick = on_dblclick.into_handler();
            view! {
                <button
                    type="button"
                    aria_label=label
                    disabled=disabled
                    on:click=on_click
                    on:dblclick=on_dblclick
                >
                    <Icon src=icon inverted=inverted_icon/>
                    {title}
                </button>
            }
            .into_any()
        }
    }
}

#[component]
pub fn sprite(data: SpriteData) -> impl IntoView {
    let SpriteData {
        src,
        alt,
        pos,
        size,
    } = data;
    view! {
        <img
            src=src
            alt=alt
            style=format!(
                "object-fit:none;object-position:{}px {}px;width:{}px;height:{}px;",
                pos.x,
                pos.y,
                size.w,
                size.h,
            )
        />
    }
}
