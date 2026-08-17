use crate::{BrowserCenter, Dimensions, WindowData};
use leptos::prelude::*;
use leptos_use::core::Position;
use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
    ops,
};

impl WindowData {
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
impl Eq for WindowData {}
impl PartialEq for WindowData {
    fn eq(&self, other: &Self) -> bool {
        self.icon == other.icon && self.title == other.title
    }
}
impl Hash for WindowData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.icon.hash(state);
        self.title.hash(state);
    }
}
// End WindowData

/// Stores a set of all available windows' [`WindowData`]
#[derive(Clone)]
pub struct Windows(pub RwSignal<HashSet<WindowData>>);
impl ops::Deref for Windows {
    type Target = RwSignal<HashSet<WindowData>>;

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
