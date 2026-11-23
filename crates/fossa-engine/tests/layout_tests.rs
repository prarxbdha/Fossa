use fossa_engine::dom::node::Node;
use fossa_engine::layout::engine::layout;

#[test]
fn layout_uses_viewport_width() {
    let root = layout(&Node::Document, 800.0);
    assert_eq!(root.width, 800.0);
}
