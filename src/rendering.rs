use ggez::graphics::Color;

pub const WHITE: Color = Color { r: 0.921, g: 0.859, b: 0.698, a: 1.0};
pub const BLACK: Color = Color { r: 0.157, g: 0.157, b: 0.157, a: 1.0 };

#[derive(Debug, Copy, Clone)]
pub enum RenderType {
    Glyph { id: char },
    Image { id: &'static str },
}
