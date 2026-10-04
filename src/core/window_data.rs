use crate::{BrowserCenter, Dimensions};
use leptos::{html, prelude::*};
use leptos_use::core::Position;
use std::ops;

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
    /// z level of the window depending on when it was last active
    pub z_index: RwSignal<usize>,
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

    /// [`WindowData`] builder function \
    /// Sets the `icon`'s path for the window
    pub fn icon(mut self, path: &'static str) -> Self {
        self.icon = path;
        self
    }

    /// [`WindowData`] builder function \
    /// Sets the `title` for the window
    pub fn title(mut self, title: &'static str) -> Self {
        self.title = title;
        self
    }

    /// [`WindowData`] builder function \
    /// Sets the [`Dimensions`] for the window
    pub fn dimensions(self, dim: Dimensions) -> Self {
        *self.dimensions.write() = dim;
        self
    }

    /// [`WindowData`] builder function \
    /// Sets the `initial_position` for the window `Default: (0,0)`
    pub fn position(mut self, pos: Position) -> Self {
        self.initial_position = pos;
        *self.set_position.write() = pos;
        self
    }

    /// [`WindowData`] builder function \
    /// Sets the `initial_position` for the window to be centered in the browser
    pub fn centered(mut self) -> Self {
        let Position { x, y } = *expect_context::<BrowserCenter>().read_untracked();
        let Dimensions { w, h } = *self.dimensions.read_untracked();
        let centered = Position {
            x: x - w / 2.0,
            y: y - h / 2.0,
        };
        self.initial_position = centered;
        *self.set_position.write() = centered;
        self
    }

    /// [`WindowData`] builder function \
    /// Sets whether the window starts open or closed `Default: true`
    pub fn open(self, is_open: bool) -> Self {
        self.is_open.set(is_open);
        self
    }

    /// [`WindowData`] builder function \
    /// Sets whether the window starts minimized or not `Default: false`
    pub fn minimized(self, is_minimized: bool) -> Self {
        self.is_minimized.set(is_minimized);
        self
    }

    /// [`WindowData`] builder function \
    /// Sets whether the window will create a desktop item for itself `Default: true`
    pub fn create_desktop_item(mut self, create: bool) -> Self {
        self.desktop_item = create;
        self
    }
}
// End WindowData

/// Stores a set of all available windows' [`WindowData`]
#[derive(Clone)]
pub struct Windows(pub RwSignal<Vec<WindowData>>);
impl Windows {
    pub fn add_window(&self, window: WindowData) {
        self.0.update(|windows| {
            windows.push(window);
        });
    }

    pub fn update_active_window(&self, window: NodeRef<html::Div>) {
        let index = self.find_window_index(window).unwrap();
        self.0.update(|windows| {
            windows[..=index].rotate_right(1);
        });
        self.update_z_indexes();
    }

    pub fn find_window_index(&self, window: NodeRef<html::Div>) -> Option<usize> {
        let window = window.get();
        self.0
            .read()
            .iter()
            .position(|wd| wd.node_ref.get() == window)
    }

    fn update_z_indexes(&self) {
        for (index, window) in self.0.read().iter().rev().enumerate() {
            window.z_index.set(index);
        }
    }
}
impl ops::Deref for Windows {
    type Target = RwSignal<Vec<WindowData>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ops::DerefMut for Windows {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
// End Windows
