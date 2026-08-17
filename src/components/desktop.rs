use super::Button;
use crate::{DesktopItem, DesktopItemFunction, DesktopItems};
use leptos::prelude::*;

/// Creates the area in main where [`DesktopItem`]s like window shorcuts or external links are be displayed
#[component]
pub fn desktop() -> impl IntoView {
    let items = expect_context::<DesktopItems>();
    view! {
        <div id="desktop" aria_label="desktop">
            <For
                each=move || items()
                key=|item| item.title
                children=move |item| view! { <DesktopItem item/> }
            />
        </div>
    }
}

/// Creates a desktop item that either opens a closed window or opens an external website with a double click
#[component]
fn desktop_item(item: DesktopItem) -> impl IntoView {
    let DesktopItem { icon, title, func } = item;
    let label = format!("desktop item: {title}");
    use DesktopItemFunction::*;
    match func {
        Window(is_open) => view! { <Button icon title label on_dblclick=move |_| is_open(true)/> },
        ExternalLink(link) => view! { <Button icon title label link/> },
    }
}
