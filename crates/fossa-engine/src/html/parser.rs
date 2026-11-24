use crate::dom::node::Node;
use crate::html::tokenizer::{Attribute, Token, tokenize};

const VOID_TAGS: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source",
    "track", "wbr",
];

pub fn parse(html: &str) -> Node {
    let tokens = tokenize(html);
    let mut position = 0usize;
    let children = parse_children(&tokens, &mut position, "");

    if children.is_empty() {
        Node::Document
    } else {
        Node::Element {
            name: "#document".to_owned(),
            attributes: Vec::new(),
            children,
        }
    }
}

fn parse_children(tokens: &[Token], position: &mut usize, parent: &str) -> Vec<Node> {
    let mut children = Vec::new();

    while *position < tokens.len() {
        match &tokens[*position] {
            Token::EndTag(name) => {
                *position += 1;
                if name == parent {
                    return children;
                }
            }
            Token::StartTag {
                name,
                attributes,
                self_closing,
            } => {
                let tag = name.clone();
                let attributes = attribute_pairs(attributes);
                let self_closing = *self_closing;
                *position += 1;

                if VOID_TAGS.contains(&tag.as_str()) || self_closing {
                    children.push(Node::Element {
                        name: tag,
                        attributes,
                        children: Vec::new(),
                    });
                } else {
                    let nested = parse_children(tokens, position, &tag);
                    children.push(Node::Element {
                        name: tag,
                        attributes,
                        children: nested,
                    });
                }
            }
            Token::Text(text) | Token::RawText(text) => {
                let trimmed = text.trim();
                if !trimmed.is_empty() {
                    children.push(Node::Text(trimmed.to_owned()));
                }
                *position += 1;
            }
        }
    }

    children
}

fn attribute_pairs(attributes: &[Attribute]) -> Vec<(String, String)> {
    attributes
        .iter()
        .map(|attribute| (attribute.name.clone(), attribute.value.clone()))
        .collect()
}
