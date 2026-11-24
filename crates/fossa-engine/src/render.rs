use crate::css::cascade::compute_styles;
use crate::html::parser::parse;
use crate::layout::engine::layout;
use crate::paint::display_list::DisplayList;
use crate::paint::painter::paint;

/// Runs the complete Fossa document rendering pipeline in one place so both
/// the Tauri bridge and integration tests use the same stage order.
pub fn render_document(html: &str, viewport_width: f32) -> DisplayList {
    let dom = parse(html);
    let stylesheet = crate::css::styles_from_dom(&dom);
    let styled = compute_styles(&dom, &stylesheet);
    let layout_tree = layout(&styled, viewport_width);
    paint(&layout_tree)
}
