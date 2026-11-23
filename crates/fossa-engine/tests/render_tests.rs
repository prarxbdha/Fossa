use fossa_engine::dom::node::Node;
use fossa_engine::html::parser::parse;
use fossa_engine::html::tokenizer::{Token, tokenize};
use fossa_engine::layout::engine::layout;
use fossa_engine::paint::display_list::DisplayItem;
use fossa_engine::paint::painter::paint;

#[test]
fn tokenizer_extracts_tags_and_text() {
    assert_eq!(
        tokenize("<h1>Hello</h1>"),
        vec![
            Token::StartTag("h1".to_owned()),
            Token::Text("Hello".to_owned()),
            Token::EndTag("h1".to_owned()),
        ]
    );
}

#[test]
fn parser_builds_a_document_tree() {
    let dom = parse("<html><body><p>Hello</p></body></html>");
    let Node::Element { name, children, .. } = dom else {
        panic!("expected a document element");
    };
    assert_eq!(name, "#document");
    assert_eq!(children.len(), 1);
}

#[test]
fn render_pipeline_produces_text_display_items() {
    let html = "<!doctype html><html><head><title>Ignored</title></head><body><h1>Hello, Fossa</h1></body></html>";
    let display_list = paint(&layout(&parse(html), 800.0));

    let text = display_list.items.iter().find_map(|item| match item {
        DisplayItem::Text { text, .. } => Some(text.as_str()),
        _ => None,
    });

    assert_eq!(text, Some("Hello, Fossa"));
    assert!(
        display_list
            .items
            .iter()
            .all(|item| !matches!(item, DisplayItem::Text { text, .. } if text == "Ignored"))
    );
}

#[test]
fn long_text_wraps_within_viewport() {
    let html = "<p>one two three four five six seven eight nine ten eleven twelve</p>";
    let display_list = paint(&layout(&parse(html), 200.0));

    let lines: Vec<&str> = display_list
        .items
        .iter()
        .filter_map(|item| match item {
            DisplayItem::Text { text, .. } => Some(text.as_str()),
            _ => None,
        })
        .collect();

    assert!(
        lines.len() > 1,
        "expected several wrapped lines, got {:?}",
        lines
    );
    assert_eq!(lines.join(" ").split_whitespace().count(), 12);
    assert!(display_list.items.iter().all(|item| match item {
        DisplayItem::Text { width, .. } => *width <= 200.0,
        _ => true,
    }));
}

#[test]
fn bundled_fixtures_render_to_text_display_items() {
    let fixtures = [
        include_str!("../../../fixtures/basic.html"),
        include_str!("../../../fixtures/typography.html"),
        include_str!("../../../fixtures/styles.html"),
        include_str!("../../../fixtures/links.html"),
    ];

    for html in fixtures {
        let display_list = paint(&layout(&parse(html), 800.0));
        assert!(
            display_list.items.iter().any(|item| matches!(
                item,
                DisplayItem::Text { text, .. } if !text.trim().is_empty()
            )),
            "fixture produced no text display items"
        );
    }
}
