use fossa_engine::css::cascade::compute_styles;
use fossa_engine::css::parser::parse as parse_css;
use fossa_engine::dom::node::Node;
use fossa_engine::html::parser::parse as parse_html;
use fossa_engine::layout::box_tree::LayoutBox;
use fossa_engine::layout::engine::layout;

#[test]
fn layout_uses_viewport_width() {
    let dom = Node::Document;
    let styled = compute_styles(&dom, &parse_css(""));
    let root = layout(&styled, 800.0);
    assert_eq!(root.width, 800.0);
}

fn find_text_box(box_node: &LayoutBox) -> Option<&LayoutBox> {
    if box_node.text.is_some() {
        return Some(box_node);
    }
    box_node.children.iter().find_map(find_text_box)
}

#[test]
fn layout_uses_computed_font_size_and_color() {
    let dom = parse_html("<style>p{color:#24c8db;font-size:32px}</style><p>Hello</p>");
    let stylesheet = fossa_engine::css::styles_from_dom(&dom);
    let styled = compute_styles(&dom, &stylesheet);
    let layout_tree = layout(&styled, 800.0);
    let text = find_text_box(&layout_tree).expect("text box");

    assert_eq!(text.font_size, 32.0);
    assert_eq!(text.height, 32.0 * 1.4);
    assert_eq!(
        text.color,
        fossa_engine::paint::display_list::Color::new(36, 200, 219, 255)
    );
}

#[test]
fn non_finite_viewport_width_is_sanitized() {
    let dom = Node::Document;
    let styled = compute_styles(&dom, &parse_css(""));
    let root = layout(&styled, f32::NAN);
    assert_eq!(root.width, 0.0);
    assert!(root.width.is_finite());
}
