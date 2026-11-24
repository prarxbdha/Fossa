use crate::paint::display_list::Color;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LayoutBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub children: Vec<LayoutBox>,
    pub text: Option<String>,
    pub font_size: f32,
    pub color: Color,
}
