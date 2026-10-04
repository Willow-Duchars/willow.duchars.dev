use std::ops;

use crate::WindowData;
use leptos::{html, prelude::*};

#[derive(Clone, Copy)]
pub struct TaskbarItem {
    pub icon: &'static str,
    pub title: &'static str,
    pub node_ref: NodeRef<html::Div>,
    pub is_open: RwSignal<bool>,
    pub is_minimized: RwSignal<bool>,
}

impl TaskbarItem {
    pub fn new(
        icon: &'static str,
        title: &'static str,
        node_ref: NodeRef<html::Div>,
        is_open: RwSignal<bool>,
        is_minimized: RwSignal<bool>,
    ) -> Self {
        Self {
            icon,
            title,
            node_ref,
            is_open,
            is_minimized,
        }
    }
}

impl From<WindowData> for TaskbarItem {
    fn from(window: WindowData) -> Self {
        let WindowData {
            icon,
            title,
            node_ref,
            is_open,
            is_minimized,
            ..
        } = window;
        Self {
            icon,
            title,
            node_ref,
            is_open,
            is_minimized,
        }
    }
}

/// Stores the set of all [`TaskbarItem`]s \
/// Used for simplifying type signatures and for implementing a default set of items
#[derive(Clone, Default)]
pub struct TaskbarItems(pub RwSignal<Vec<TaskbarItem>>);
impl TaskbarItems {
    pub fn add_item(&self, window: WindowData) {
        self.0.update(|items| {
            items.push(window.into());
        });
    }
}
impl ops::Deref for TaskbarItems {
    type Target = RwSignal<Vec<TaskbarItem>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ops::DerefMut for TaskbarItems {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
