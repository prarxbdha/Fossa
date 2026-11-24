#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub const DEFAULT_TEXT_COLOR: Color = Color {
    r: 231,
    g: 233,
    b: 237,
    a: 255,
};

impl Default for Color {
    fn default() -> Self {
        DEFAULT_TEXT_COLOR
    }
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayItem {
    Rect {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: Color,
    },
    Text {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        text: String,
        color: Color,
        font_size: f32,
    },
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DisplayList {
    pub items: Vec<DisplayItem>,
}
