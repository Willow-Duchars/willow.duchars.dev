use super::{Button, Icon};
use crate::{constants::icons, BrowserDimensions, DesktopItems, Dimensions, WindowData, Windows};
use leptos::{html, prelude::*};
use leptos_use::core::Position;

/// Creates a new window \
/// Use the [`WindowData`] builder functions to build up data for the window
#[component]
pub fn window(
    data: WindowData,
    #[prop(optional)] id: &'static str,
    #[prop(optional)] disable_maximize: bool,
    children: ChildrenFn,
) -> impl IntoView {
    let WindowData {
        icon,
        title,
        node_ref,
        dimensions,
        position,
        is_minimized,
        is_maximized,
        is_open,
        desktop_item,
        ..
    } = data;

    expect_context::<Windows>().update(|w| {
        w.insert(data);
    });

    if desktop_item {
        expect_context::<DesktopItems>().update(|items| {
            items.insert(data.into());
        });
    }

    let browser_dimensions = expect_context::<BrowserDimensions>();
    let window_ref = NodeRef::<html::Div>::new();

    Effect::new(move || {
        let Dimensions { w, h } = match is_maximized() {
            true => {
                update_displayed_position(window_ref, 0.0, 0.0);
                *browser_dimensions.read()
            }
            false => *dimensions.read(),
        };
        update_displayed_dimensions(window_ref, w, h);
    });

    Effect::new(move || {
        let Position { x, y } = *position.read();
        update_displayed_position(window_ref, x, y);
    });

    let close = move |_| is_open(false);
    let minimize = move |_| is_minimized(true);
    let maximize = move |_| {
        is_maximized.update(|maxed| {
            if *maxed {
                let Position { x, y } = *position.read();
                update_displayed_position(window_ref, x, y);
            }
            *maxed = !*maxed;
        })
    };

    view! {
        <Show when=move || is_open() && !is_minimized() fallback=|| ()>
            <div node_ref=window_ref class="window" id=id aria_label=format!("{title} window")>
                <div class="window-bar">
                    <div node_ref=node_ref>
                        <Icon src=icon/>
                        <h1>{title}</h1>
                    </div>
                    <Button icon=icons::MINIMIZE label="minimize window" on_click=minimize/>
                    <Button
                        icon=icons::MAXIMIZE
                        label="maximize window"
                        on_click=maximize
                        disabled=disable_maximize
                    />
                    <Button icon=icons::CLOSE label="close window" on_click=close/>
                </div>
                <div>{children()}</div>
            </div>
        </Show>
    }
}

/// Updates the referenced item's css styling to change its position on screen
fn update_displayed_position(node_ref: NodeRef<html::Div>, x: f64, y: f64) {
    if let Some(nr) = node_ref.get() {
        let _ = (*nr).style().set_property("left", &format!("{x}px"));
        let _ = (*nr).style().set_property("top", &format!("{y}px"));
    }
}

/// Update the referenced item's css styling to change its on screen dimensions
fn update_displayed_dimensions(node_ref: NodeRef<html::Div>, w: f64, h: f64) {
    if let Some(nr) = node_ref.get() {
        let _ = (*nr).style().set_property("width", &format!("{w}px"));
        let _ = (*nr).style().set_property("height", &format!("{h}px"));
    }
}
