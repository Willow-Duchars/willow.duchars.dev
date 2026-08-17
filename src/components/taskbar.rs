use super::Button;
use crate::{constants::icons, WindowData, Windows};
use leptos::prelude::*;

/// Creates the area in the footer where the taskbar items are displayed
#[component]
pub fn taskbar() -> impl IntoView {
    let windows = expect_context::<Windows>();
    view! {
        <div id="taskbar" aria_label="taskbar">
            <MenuButton/>
            <For
                each=move || windows()
                key=|window| window.title
                children=move |data| view! { <TaskbarItem data/> }
            />
        </div>
    }
}

/// Creates a new taskbar item that is related to the window whose data was provided
#[component]
fn taskbar_item(data: WindowData) -> impl IntoView {
    view! {
        <Show when=move || data.is_open.get() fallback=|| ()>
            <Button
                icon=data.icon
                label=format!("taskbar item: {}", data.title.to_lowercase())
                on_click=move |_| data.is_minimized.set(false)
                on_dblclick=move |_| data.reset_position()
            />
        </Show>
    }
}

#[component]
pub fn menu_button() -> impl IntoView {
    view! { <Button icon=icons::MENU label="main menu" inverted_icon=true/> }
}
