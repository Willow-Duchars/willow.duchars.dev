use crate::{DesktopItem, DesktopItems, Dimensions, WindowData, Windows};
use leptos::{html, prelude::*};
use leptos_use::{use_draggable_with_options, UseDraggableOptions, UseDraggableReturn};
use std::collections::HashSet;

impl Default for Dimensions {
    fn default() -> Self {
        Self { w: 400.0, h: 250.0 }
    }
}

impl Default for Windows {
    fn default() -> Self {
        Self(RwSignal::new(HashSet::default()))
    }
}

impl Default for WindowData {
    fn default() -> Self {
        let node_ref = NodeRef::<html::Div>::new();
        let UseDraggableReturn {
            position,
            set_position,
            ..
        } = use_draggable_with_options(
            node_ref,
            UseDraggableOptions::default().prevent_default(true),
        );
        Self {
            icon: "",
            title: "Window",
            node_ref,
            dimensions: Default::default(),
            position,
            set_position,
            initial_position: Default::default(),
            is_minimized: RwSignal::new(false),
            is_maximized: RwSignal::new(false),
            is_open: RwSignal::new(true),
            desktop_item: true,
        }
    }
}
// End Default WindowData

impl Default for DesktopItems {
    fn default() -> Self {
        use crate::constants::{external_links, icons};
        Self(RwSignal::new(HashSet::from([
            DesktopItem::new(icons::GITHUB, "Github", external_links::GITHUB),
            DesktopItem::new(icons::LINKEDIN, "LinkedIn", external_links::LINKEDIN),
        ])))
    }
}
