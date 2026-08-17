use crate::Dimensions;
use leptos_use::core::Position;

pub struct SpriteData {
    pub src: &'static str,
    pub alt: &'static str,
    pub pos: Position,
    pub size: Dimensions,
}
