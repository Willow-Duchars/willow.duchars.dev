// Modules
pub mod components;
mod core;
// Re-exports
pub use core::browser::{self, BrowserCenter, BrowserDimensions};
pub use core::constants;
pub use core::desktop_items::{DesktopItem, DesktopItemFunction, DesktopItems};
pub use core::sprite_data::SpriteData;
pub use core::taskbar_items::{TaskbarItem, TaskbarItems};
pub use core::window_data::{WindowData, Windows};
pub mod prelude {
    pub use crate::Dimensions;
    pub use crate::WindowData;
    pub use crate::components::Window;
    pub use crate::constants::icons;
}

/// Semantic struct for dimension data
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dimensions {
    pub w: f64,
    pub h: f64,
}
