use crate::layout::box_tree::LayoutBox;
use crate::paint::display_list::{Color, DisplayItem, DisplayList};

const TEXT_COLOR: Color = Color {
    r: 231,
    g: 233,
    b: 237,
    a: 255,
};

/// Layout already places boxes in content coordinates (including padding), so
/// painting is a direct translation into display items.
pub fn paint(root: &LayoutBox) -> DisplayList {
    let mut items = Vec::new();
    paint_box(root, &mut items);
    DisplayList { items }
}

fn paint_box(layout_box: &LayoutBox, items: &mut Vec<DisplayItem>) {
    if let Some(text) = &layout_box.text {
        items.push(DisplayItem::Text {
            x: layout_box.x,
            y: layout_box.y,
            width: layout_box.width,
            height: layout_box.height,
            text: text.clone(),
            color: TEXT_COLOR,
            font_size: layout_box.font_size,
        });
    }

    for child in &layout_box.children {
        paint_box(child, items);
    }
}
