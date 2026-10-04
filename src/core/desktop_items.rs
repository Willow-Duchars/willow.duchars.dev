use crate::WindowData;
use leptos::{html, prelude::*};
use std::ops;

/// Stores data about desktop items and has function depending on whether its a window item or an external link
#[derive(Clone, Copy)]
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
            node_ref,
            ..
        } = window;
        Self {
            icon,
            title,
            func: (is_open, node_ref).into(),
        }
    }
}

/// Stores the set of all [`DesktopItem`]s \
/// Used for simplifying type signatures and for implementing a default set of items
#[derive(Clone)]
pub struct DesktopItems(pub RwSignal<Vec<DesktopItem>>);
impl DesktopItems {
    pub fn add_item(&self, window: WindowData) {
        self.0.update(|items| {
            items.push(window.into());
        });
    }
}
impl ops::Deref for DesktopItems {
    type Target = RwSignal<Vec<DesktopItem>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ops::DerefMut for DesktopItems {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
// End DesktopItems

/// Sum type for desktop item functions \
/// Created from a `RwSignal<bool>` or `&'static str`
#[derive(Clone, Copy)]
pub enum DesktopItemFunction {
    Window(RwSignal<bool>, NodeRef<html::Div>),
    ExternalLink(&'static str),
}
impl From<(RwSignal<bool>, NodeRef<html::Div>)> for DesktopItemFunction {
    fn from((is_open, node_ref): (RwSignal<bool>, NodeRef<html::Div>)) -> Self {
        Self::Window(is_open, node_ref)
    }
}
impl From<&'static str> for DesktopItemFunction {
    fn from(link: &'static str) -> Self {
        Self::ExternalLink(link)
    }
}
// End DesktopItemFunction
