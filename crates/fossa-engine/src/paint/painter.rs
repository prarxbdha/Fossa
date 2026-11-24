use crate::layout::box_tree::LayoutBox;
use crate::paint::display_list::{DisplayItem, DisplayList};

/// Layout already places boxes in content coordinates (including padding) and
/// carries resolved styles, so painting is a direct translation into items.
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
            color: layout_box.color,
            font_size: layout_box.font_size,
        });
    }

    for child in &layout_box.children {
        paint_box(child, items);
    }
}
