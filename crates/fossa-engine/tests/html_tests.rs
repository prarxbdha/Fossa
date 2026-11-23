use fossa_engine::dom::node::Node;
use fossa_engine::html::parser::parse;

#[test]
fn empty_html_produces_document_node() {
    assert_eq!(parse(""), Node::Document);
}
