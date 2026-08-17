// Modules
pub mod components;
mod core;
// Re-exports
pub use core::browser::{self, BrowserCenter, BrowserDimensions};
pub use core::constants;
pub use core::desktop_items::{DesktopItemFunction, DesktopItems};
pub use core::sprite_data::SpriteData;
pub use core::window_data::Windows;
// Imports
use leptos::{html, prelude::*};
use leptos_use::core::Position;

/// Semantic struct for dimension data
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dimensions {
    pub w: f64,
    pub h: f64,
}

/// Stores data about the window and reactive signals to control the window
#[derive(Clone, Copy)]
pub struct WindowData {
    /// static str with icon path
    pub icon: &'static str,
    /// static str with title
    pub title: &'static str,
    /// NodeRef to window bar div
    pub node_ref: NodeRef<html::Div>,
    /// dimensions of the window
    pub dimensions: RwSignal<Dimensions>,
    /// reactive position of the window
    pub position: Signal<Position>,
    /// setter for the window position
    pub set_position: WriteSignal<Position>,
    /// initial starting position of the window
    pub initial_position: Position,
    /// reactive read & write signal for whether the window is minimized
    pub is_minimized: RwSignal<bool>,
    /// reactive read & write signal for whether the window is maximized
    pub is_maximized: RwSignal<bool>,
    /// reactive read & write signal for whether the window is open
    pub is_open: RwSignal<bool>,
    /// whether the window should create a desktop item
    pub desktop_item: bool,
}

impl WindowData {
    /// Creates a new [`WindowData`] \
    /// You can use the various builder functions to customize the window
    /// ```rust
    /// // Creating a new window
    /// let data = WindowData::new("public/icon.svg", "My Custom Window")
    ///     .dimensions(Dimensions{ w: 1280, h: 720 })
    ///     .centered();
    /// view! {
    ///     <Window data>
    ///         <h1>"This is the new window"</h1>
    ///         <p>"You can put your content here"<p>
    ///     </Window>
    /// }
    /// ```
    pub fn new(icon: &'static str, title: &'static str) -> Self {
        Self {
            icon,
            title,
            ..Default::default()
        }
    }

    /// Resets the window's position to its `initial_position`
    pub fn reset_position(&self) {
        *self.set_position.write() = self.initial_position;
    }
}

/// Stores data about desktop items and has function depending on whether its a window item or an external link
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct DesktopItem {
    pub icon: &'static str,
    pub title: &'static str,
    pub func: DesktopItemFunction,
}

impl DesktopItem {
    /// Constructs a new item with an icon, a title, and a function that is either a `&'static str` which holds an external url or a window's `is_open signal`
    pub fn new(
        icon: &'static str,
        title: &'static str,
        func: impl Into<DesktopItemFunction>,
    ) -> Self {
        let func = func.into();
        Self { icon, title, func }
    }
}

impl From<WindowData> for DesktopItem {
    fn from(window: WindowData) -> Self {
        let WindowData {
            icon,
            title,
            is_open,
            ..
        } = window;
        Self {
            icon,
            title,
            func: is_open.into(),
        }
    }
}
