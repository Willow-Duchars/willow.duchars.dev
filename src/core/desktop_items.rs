use crate::DesktopItem;
use leptos::prelude::*;
use std::{collections::HashSet, ops};

/// Stores the set of all [`DesktopItem`]s \
/// Used for simplifying type signatures and for implementing a default set of items
#[derive(Clone)]
pub struct DesktopItems(pub RwSignal<HashSet<DesktopItem>>);
impl ops::Deref for DesktopItems {
    type Target = RwSignal<HashSet<DesktopItem>>;

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
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum DesktopItemFunction {
    Window(RwSignal<bool>),
    ExternalLink(&'static str),
}
impl From<RwSignal<bool>> for DesktopItemFunction {
    fn from(window: RwSignal<bool>) -> Self {
        Self::Window(window)
    }
}
impl From<&'static str> for DesktopItemFunction {
    fn from(link: &'static str) -> Self {
        Self::ExternalLink(link)
    }
}
// End DesktopItemFunction
