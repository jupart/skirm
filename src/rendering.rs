#[derive(Debug, Copy, Clone)]
pub enum RenderType {
    Glyph { id: &'static str },
    Image { id: &'static str },
}
