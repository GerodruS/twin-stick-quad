#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TextureWrapper {
    pub index: usize,
}

impl TextureWrapper {
    pub fn new(index: usize) -> TextureWrapper {
        TextureWrapper { index }
    }
}
