#![allow(dead_code)]
use crate::{constants::styling, Dimensions};
use leptos::{ev, prelude::*};
use leptos_use::{core::Position, use_event_listener, use_window};
use std::ops;

/// Stores the calculated [`Position`] of the browser's centerpoint \
/// Needs to be initialized using [`setup_browser_center_listener()`] before use
#[derive(Clone, Copy)]
pub struct BrowserCenter(pub ReadSignal<Position>);
impl From<ReadSignal<Position>> for BrowserCenter {
    fn from(value: ReadSignal<Position>) -> Self {
        Self(value)
    }
}
impl ops::Deref for BrowserCenter {
    type Target = ReadSignal<Position>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ops::DerefMut for BrowserCenter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
// End BrowserCenter

/// Stores the calculated [`Dimensions`] of the browser's viewport \
/// Needs to be initialized using [`setup_browser_dimensions_listener()`] before use
#[derive(Clone, Copy)]
pub struct BrowserDimensions(pub ReadSignal<Dimensions>);
impl From<ReadSignal<Dimensions>> for BrowserDimensions {
    fn from(value: ReadSignal<Dimensions>) -> Self {
        Self(value)
    }
}
impl ops::Deref for BrowserDimensions {
    type Target = ReadSignal<Dimensions>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ops::DerefMut for BrowserDimensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
// End BrowserDimensions

/// Sets up the event listener that updates the browser centerpoint calculation
pub fn setup_browser_center_listener() {
    let (browser_center, set_browser_center) = signal(calc_browser_center());
    let update_browser_center = move |_| set_browser_center(calc_browser_center());
    let _ = use_event_listener(use_window(), ev::resize, update_browser_center);
    provide_context(BrowserCenter::from(browser_center));
}

/// Sets up the event listener that updates the browser's dimensions calculation
pub fn setup_browser_dimensions_listener() {
    let (window_dimensions, set_window_dimensions) = signal(calc_browser_dimensions());
    let update_window_dimensions = move |_| set_window_dimensions(calc_browser_dimensions());
    let _ = use_event_listener(use_window(), ev::resize, update_window_dimensions);
    provide_context(BrowserDimensions::from(window_dimensions));
}

/// Calculates the browser window's viewport's center minus the taskbar
fn calc_browser_center() -> Position {
    let Dimensions { w, h } = calc_browser_dimensions();
    Position {
        x: w / 2.0,
        y: h / 2.0,
    }
}

/// Calculates the browser window's viewport's dimensions minus the taskbar
fn calc_browser_dimensions() -> Dimensions {
    Dimensions {
        w: get_browser_width(),
        h: get_browser_height(),
    }
}

/// Gets the browser window's viewport's width
fn get_browser_width() -> f64 {
    window()
        .inner_width()
        .expect("should have a value")
        .as_f64()
        .expect("should be a number")
}

/// Gets the browser window's viewport's height, subtracting the taskbar's height
fn get_browser_height() -> f64 {
    window()
        .inner_height()
        .expect("should have a value")
        .as_f64()
        .expect("should be a number")
        - styling::TASKBAR_TOTAL_HEIGHT
}

/// Sets up css variables for taskbar and main sizing
pub fn setup_taskbar_styling() -> String {
    use styling::*;
    format!(
        ":root{{
    --taskbar-border-width:{TASKBAR_BORDER_WIDTH}px;
    --taskbar-total-padding:{TASKBAR_TOTAL_PADDING}px;
    --taskbar-height:{TASKBAR_HEIGHT}px;
    --taskbar-total-height:{TASKBAR_TOTAL_HEIGHT}px;
}}"
    )
}
