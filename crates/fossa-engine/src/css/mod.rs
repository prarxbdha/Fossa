pub mod cascade;
pub mod parser;
pub mod selector;
pub mod style;
pub mod tokenizer;

use crate::css::style::StyleSheet;
use crate::dom::node::Node;

/// Collects `<style>` element contents in document order and parses them as CSS.
pub fn styles_from_dom(dom: &Node) -> StyleSheet {
    let mut sources = Vec::new();
    collect_style_text(dom, &mut sources);

    let mut combined = StyleSheet::default();
    for source in sources {
        combined.rules.extend(parser::parse(&source).rules);
    }
    combined
}

fn collect_style_text(node: &Node, sources: &mut Vec<String>) {
    match node {
        Node::Element { name, children, .. } => {
            if name == "style" {
                let css: String = children
                    .iter()
                    .filter_map(|child| match child {
                        Node::Text(text) => Some(text.as_str()),
                        _ => None,
                    })
                    .collect();
                sources.push(css);
                return;
            }
            for child in children {
                collect_style_text(child, sources);
            }
        }
        Node::Document | Node::Text(_) => {}
    }
}
