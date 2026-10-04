use super::Button;
use crate::{constants::icons, TaskbarItem, TaskbarItems, Windows};
use leptos::prelude::*;

/// Creates the area in the footer where the taskbar items are displayed
#[component]
pub fn taskbar() -> impl IntoView {
    let items = expect_context::<TaskbarItems>();
    view! {
        <div id="taskbar" aria_label="taskbar">
            <MenuButton/>
            <For
                each=move || items()
                key=|item| item.title
                children=move |item| view! { <TaskbarItem item/> }
            />
        </div>
    }
}

/// Creates a new taskbar item that is related to the window whose data was provided
#[component]
fn taskbar_item(item: TaskbarItem) -> impl IntoView {
    view! {
        <Show when=move || item.is_open.get() fallback=|| ()>
            <Button
                icon=item.icon
                label=format!("taskbar item: {}", item.title.to_lowercase())
                on_click=move |_| {
                    expect_context::<Windows>().update_active_window(item.node_ref);
                    item.is_minimized.set(false);
                }

                on_dblclick=move |_| {
                    let windows = expect_context::<Windows>();
                    let index = windows.find_window_index(item.node_ref).unwrap();
                    windows
                        .with(move |windows| {
                            windows[index].reset_position();
                        });
                }
            />

        </Show>
    }
}

#[component]
pub fn menu_button() -> impl IntoView {
    view! { <Button icon=icons::MENU label="main menu" inverted_icon=true/> }
}
