use fossa_engine::dom::node::Node;
use fossa_engine::html::parser::parse;
use fossa_engine::html::tokenizer::{Attribute, Token, tokenize};
use fossa_engine::paint::display_list::{Color, DisplayItem, DisplayList};
use fossa_engine::render::render_document;

fn text_items(list: &DisplayList) -> Vec<&DisplayItem> {
    list.items
        .iter()
        .filter(|item| matches!(item, DisplayItem::Text { .. }))
        .collect()
}

fn text_item<'a>(list: &'a DisplayList, text: &str) -> &'a DisplayItem {
    list.items
        .iter()
        .find(|item| matches!(item, DisplayItem::Text { text: actual, .. } if actual == text))
        .unwrap_or_else(|| panic!("missing text item {text:?}"))
}

#[test]
fn tokenizer_extracts_tags_and_text() {
    assert_eq!(
        tokenize("<h1>Hello</h1>"),
        vec![
            Token::StartTag {
                name: "h1".to_owned(),
                attributes: Vec::<Attribute>::new(),
                self_closing: false,
            },
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
    let display_list = render_document(html, 800.0);

    assert!(text_items(&display_list).iter().any(
        |item| matches!(item, DisplayItem::Text { text, .. } if text.as_str() == "Hello, Fossa")
    ));
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
    let display_list = render_document(html, 200.0);
    let lines = text_items(&display_list);

    assert!(
        lines.len() > 1,
        "expected several wrapped lines, got {lines:?}"
    );
    assert_eq!(
        lines
            .iter()
            .flat_map(|item| match item {
                DisplayItem::Text { text, .. } => text.split_whitespace(),
                _ => unreachable!(),
            })
            .count(),
        12
    );
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
        let display_list = render_document(html, 800.0);
        assert!(
            text_items(&display_list).iter().any(
                |item| matches!(item, DisplayItem::Text { text, .. } if !text.trim().is_empty())
            ),
            "fixture produced no text display items"
        );
    }
}

#[test]
fn styles_fixture_produces_cyan_text() {
    let display_list = render_document(include_str!("../../../fixtures/styles.html"), 800.0);
    let item = text_item(&display_list, "Styled paragraph");
    let DisplayItem::Text { color, .. } = item else {
        unreachable!()
    };

    assert_eq!(*color, Color::new(36, 200, 219, 255));
}

#[test]
fn font_size_declarations_reach_paint_output() {
    let html = "<style>h1 { font-size: 32px; }</style><h1>Big</h1>";
    let display_list = render_document(html, 800.0);
    let DisplayItem::Text { font_size, .. } = text_item(&display_list, "Big") else {
        unreachable!()
    };

    assert_eq!(*font_size, 32.0);
}

#[test]
fn default_heading_sizes_remain_distinct() {
    let display_list = render_document(include_str!("../../../fixtures/typography.html"), 800.0);
    let size_of = |text: &str| match text_item(&display_list, text) {
        DisplayItem::Text { font_size, .. } => *font_size,
        _ => unreachable!(),
    };

    assert_eq!(size_of("Heading one"), 24.0);
    assert_eq!(size_of("Heading two"), 19.0);
    assert_eq!(size_of("Heading three"), 16.0);
}

#[test]
fn font_size_declarations_affect_wrapping() {
    let text = "one two three four five six seven eight nine ten";
    let plain = render_document(&format!("<p>{text}</p>"), 200.0);
    let large = render_document(
        &format!("<style>p {{ font-size: 32px; }}</style><p>{text}</p>"),
        200.0,
    );

    assert!(
        text_items(&large).len() > text_items(&plain).len(),
        "larger font size should produce more wrapped lines"
    );
}

#[test]
fn style_element_source_is_not_painted() {
    let html = "<style>p { color: red; }</style><p>Visible</p>";
    let display_list = render_document(html, 800.0);

    assert!(
        text_items(&display_list)
            .iter()
            .all(|item| !matches!(item, DisplayItem::Text { text, .. } if text.contains("color") || text.contains("p {")))
    );
    assert!(
        text_items(&display_list).iter().any(
            |item| matches!(item, DisplayItem::Text { text, .. } if text.as_str() == "Visible")
        )
    );
}

#[test]
fn multiple_style_elements_preserve_source_order_in_paint_output() {
    let html = "<style>.accent { color: red; }</style>\
                <style>.accent { color: blue; }</style>\
                <p class=\"accent\">Hello</p>";
    let display_list = render_document(html, 800.0);
    let DisplayItem::Text { color, .. } = text_item(&display_list, "Hello") else {
        unreachable!()
    };

    assert_eq!(*color, Color::new(0, 0, 255, 255));
}

#[test]
fn unsupported_css_does_not_panic() {
    let html = "<style>p:hover { color: red } p { color: not-a-color; font-size: 12em; }</style>\
                <p>Hello</p>";
    let display_list = render_document(html, 800.0);
    assert!(
        text_items(&display_list)
            .iter()
            .any(|item| matches!(item, DisplayItem::Text { text, .. } if text.as_str() == "Hello"))
    );
}
