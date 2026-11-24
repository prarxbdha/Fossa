use fossa_engine::dom::node::Node;
use fossa_engine::html::parser::parse;
use fossa_engine::html::tokenizer::{Attribute, Token, tokenize};

#[test]
fn empty_html_produces_document_node() {
    assert_eq!(parse(""), Node::Document);
}

fn first_element(node: &Node) -> &Node {
    match node {
        Node::Element { name, .. } if name != "#document" => node,
        Node::Element { children, .. } => {
            children.iter().map(first_element).next().expect("element")
        }
        Node::Document | Node::Text(_) => panic!("expected an element"),
    }
}

fn start_tag(html: &str) -> Token {
    tokenize(html)
        .into_iter()
        .find(|token| matches!(token, Token::StartTag { .. }))
        .expect("start tag")
}

#[test]
fn double_quoted_attributes_are_preserved() {
    assert_eq!(
        start_tag("<div class=\"container\">Hi</div>"),
        Token::StartTag {
            name: "div".to_owned(),
            attributes: vec![Attribute {
                name: "class".to_owned(),
                value: "container".to_owned(),
            }],
            self_closing: false,
        }
    );
}

#[test]
fn single_quoted_attributes_allow_whitespace_around_equals() {
    assert_eq!(
        start_tag("<p id = 'intro'>Hi</p>"),
        Token::StartTag {
            name: "p".to_owned(),
            attributes: vec![Attribute {
                name: "id".to_owned(),
                value: "intro".to_owned(),
            }],
            self_closing: false,
        }
    );
}

#[test]
fn unquoted_and_boolean_attributes_are_preserved() {
    let Token::StartTag {
        attributes,
        name,
        self_closing,
    } = start_tag("<a href=https://example.com disabled>Link</a>")
    else {
        panic!("expected a start tag")
    };

    assert_eq!(name, "a");
    assert!(!self_closing);
    assert_eq!(
        attributes,
        vec![
            Attribute {
                name: "href".to_owned(),
                value: "https://example.com".to_owned(),
            },
            Attribute {
                name: "disabled".to_owned(),
                value: String::new(),
            },
        ]
    );
}

#[test]
fn multiple_attributes_keep_order_and_first_duplicate_wins() {
    let Token::StartTag { attributes, .. } =
        start_tag("<div data-Value = '42' class=\"card\" id=\"main\" class=\"second\">x</div>")
    else {
        panic!("expected a start tag")
    };

    assert_eq!(
        attributes,
        vec![
            Attribute {
                name: "data-value".to_owned(),
                value: "42".to_owned(),
            },
            Attribute {
                name: "class".to_owned(),
                value: "card".to_owned(),
            },
            Attribute {
                name: "id".to_owned(),
                value: "main".to_owned(),
            },
        ]
    );
}

#[test]
fn attribute_entities_decode_once_and_values_keep_case() {
    let Token::StartTag { attributes, .. } =
        start_tag("<p data-value=\"A&amp;B &lt;C&gt; &#65; &#x1F600;\">x</p>")
    else {
        panic!("expected a start tag")
    };

    assert_eq!(
        attributes,
        vec![Attribute {
            name: "data-value".to_owned(),
            value: "A&B <C> A 😀".to_owned(),
        }]
    );
}

#[test]
fn quoted_attribute_values_may_contain_greater_than() {
    let Token::StartTag { attributes, .. } = start_tag("<a title=\"a > b\" href=x>Link</a>") else {
        panic!("expected a start tag")
    };

    assert_eq!(
        attributes,
        vec![
            Attribute {
                name: "title".to_owned(),
                value: "a > b".to_owned(),
            },
            Attribute {
                name: "href".to_owned(),
                value: "x".to_owned(),
            },
        ]
    );
}

#[test]
fn self_closing_tags_are_marked() {
    assert_eq!(
        start_tag("<img src=\"photo.png\" />"),
        Token::StartTag {
            name: "img".to_owned(),
            attributes: vec![Attribute {
                name: "src".to_owned(),
                value: "photo.png".to_owned(),
            }],
            self_closing: true,
        }
    );
}

#[test]
fn malformed_attribute_input_does_not_panic() {
    for html in [
        "<div class=\"unclosed>text",
        "<div =broken class=x>text",
        "<div class= >text",
        "<!-- unterminated comment",
    ] {
        let tokens = tokenize(html);
        assert!(
            !tokens.is_empty(),
            "malformed input should produce deterministic tokens: {html}"
        );
    }
}

#[test]
fn parser_transfers_attributes_into_the_dom() {
    let dom = parse("<p class=\"accent\" id=\"intro\">Hello</p>");
    let paragraph = first_element(&dom);

    assert_eq!(paragraph.name(), Some("p"));
    assert_eq!(paragraph.attribute("class"), Some("accent"));
    assert_eq!(paragraph.attribute("id"), Some("intro"));

    let Node::Element { children, .. } = paragraph else {
        panic!("expected an element")
    };
    assert_eq!(children, &[Node::Text("Hello".to_owned())]);
}

#[test]
fn self_closing_non_void_element_has_no_children() {
    let dom = parse("<div class=\"card\"/>");
    let element = first_element(&dom);
    assert_eq!(element.name(), Some("div"));
    assert_eq!(element.attribute("class"), Some("card"));

    let Node::Element { children, .. } = element else {
        panic!("expected an element")
    };
    assert!(children.is_empty());
}
