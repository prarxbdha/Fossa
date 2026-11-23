use crate::dom::node::Node;
use crate::html::tokenizer::{Token, tokenize};

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
            Token::StartTag(name) => {
                let tag = name.clone();
                *position += 1;

                if VOID_TAGS.contains(&tag.as_str()) {
                    children.push(Node::Element {
                        name: tag,
                        attributes: Vec::new(),
                        children: Vec::new(),
                    });
                } else {
                    let nested = parse_children(tokens, position, &tag);
                    children.push(Node::Element {
                        name: tag,
                        attributes: Vec::new(),
                        children: nested,
                    });
                }
            }
            Token::Text(text) => {
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
