use macroquad::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TextureWrapper {
    pub index: usize,
    pub rect: Rect,
}

impl TextureWrapper {
    pub fn new(index: usize, rect: Rect) -> TextureWrapper {
        TextureWrapper { index, rect }
    }
}
