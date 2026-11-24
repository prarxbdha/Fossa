use fossa_engine::css::cascade::{
    DEFAULT_FONT_SIZE, StyledNode, compute_styles, resolve_element_style,
};
use fossa_engine::css::parser::parse as parse_css;
use fossa_engine::css::styles_from_dom;
use fossa_engine::dom::node::Node;
use fossa_engine::html::parser::parse as parse_html;
use fossa_engine::paint::display_list::Color;

fn attributes(pairs: &[(&str, &str)]) -> Vec<(String, String)> {
    pairs
        .iter()
        .map(|(key, value)| ((*key).to_owned(), (*value).to_owned()))
        .collect()
}

fn find_element_style<'a>(
    node: &StyledNode<'a>,
    name: &str,
) -> Option<fossa_engine::css::cascade::ComputedStyle> {
    if node.name() == Some(name) {
        return Some(node.style);
    }
    node.children
        .iter()
        .find_map(|child| find_element_style(child, name))
}

fn find_text_style(node: &StyledNode<'_>) -> Option<fossa_engine::css::cascade::ComputedStyle> {
    if matches!(node.name(), Some("style" | "script" | "head" | "title")) {
        return None;
    }
    if matches!(node.node, Node::Text(_)) {
        return Some(node.style);
    }
    node.children.iter().find_map(find_text_style)
}

fn element_style(html: &str, name: &str) -> fossa_engine::css::cascade::ComputedStyle {
    let dom = parse_html(html);
    let stylesheet = styles_from_dom(&dom);
    let styled = compute_styles(&dom, &stylesheet);
    find_element_style(&styled, name).expect("element style")
}

fn text_style(html: &str) -> fossa_engine::css::cascade::ComputedStyle {
    let dom = parse_html(html);
    let stylesheet = styles_from_dom(&dom);
    let styled = compute_styles(&dom, &stylesheet);
    find_text_style(&styled).expect("text style")
}

#[test]
fn id_overrides_class_without_discarding_lower_specificity_properties() {
    let style = element_style(
        "<style>p{color:red;font-size:18px}.accent{color:blue}#intro{color:green}</style>\
         <p id=\"intro\" class=\"accent\">Hello</p>",
        "p",
    );
    assert_eq!(style.color, Color::new(0, 128, 0, 255));
    assert_eq!(style.font_size, 18.0);
}

#[test]
fn class_overrides_type_and_type_overrides_universal() {
    let sheet = parse_css("*{color:red}p{color:blue}.accent{color:green}");
    let inherited = Default::default();

    let paragraph =
        resolve_element_style("p", &attributes(&[("class", "accent")]), &sheet, inherited);
    assert_eq!(paragraph.color, Color::new(0, 128, 0, 255));

    let plain = resolve_element_style("p", &[], &sheet, inherited);
    assert_eq!(plain.color, Color::new(0, 0, 255, 255));

    let division = resolve_element_style("div", &[], &sheet, inherited);
    assert_eq!(division.color, Color::new(255, 0, 0, 255));
}

#[test]
fn later_equal_specificity_rule_wins() {
    let style = element_style(
        "<style>.accent{color:red}.accent{color:blue}</style><p class=\"accent\">Hello</p>",
        "p",
    );
    assert_eq!(style.color, Color::new(0, 0, 255, 255));
}

#[test]
fn properties_cascade_independently() {
    let style = element_style(
        "<style>p{color:#ff0000;font-size:18px}.accent{color:#0000ff}</style>\
         <p class=\"accent\">Hello</p>",
        "p",
    );
    assert_eq!(style.color, Color::new(0, 0, 255, 255));
    assert_eq!(style.font_size, 18.0);
}

#[test]
fn invalid_values_do_not_override_earlier_valid_values() {
    let style = element_style(
        "<style>.accent{color:not-a-color;font-size:-4px}p{color:red;font-size:18px}</style>\
         <p class=\"accent\">Hello</p>",
        "p",
    );
    assert_eq!(style.color, Color::new(255, 0, 0, 255));
    assert_eq!(style.font_size, 18.0);
}

#[test]
fn parent_color_and_font_size_are_inherited_by_text_nodes() {
    let html = "<style>div{color:#24c8db;font-size:20px}</style><div><p>Hello</p></div>";
    let paragraph = element_style(html, "p");
    let text = text_style(html);

    for style in [paragraph, text] {
        assert_eq!(style.color, Color::new(36, 200, 219, 255));
        assert_eq!(style.font_size, 20.0);
    }
}

#[test]
fn child_declarations_override_inherited_values() {
    let html = "<style>div{color:#24c8db;font-size:20px}p{color:red;font-size:12px}</style>\
                <div><p>Hello</p></div>";
    let paragraph = element_style(html, "p");
    let text = text_style(html);

    for style in [paragraph, text] {
        assert_eq!(style.color, Color::new(255, 0, 0, 255));
        assert_eq!(style.font_size, 12.0);
    }
}

#[test]
fn default_heading_sizes_remain_and_author_css_overrides_them() {
    let defaults = "<h1>One</h1><h2>Two</h2><h3>Three</h3>";
    assert_eq!(element_style(defaults, "h1").font_size, 24.0);
    assert_eq!(element_style(defaults, "h2").font_size, 19.0);
    assert_eq!(element_style(defaults, "h3").font_size, 16.0);
    assert_eq!(element_style("<p>x</p>", "p").font_size, DEFAULT_FONT_SIZE);

    let overridden = "<style>h1{font-size:32px}</style><h1>One</h1>";
    assert_eq!(element_style(overridden, "h1").font_size, 32.0);
}

#[test]
fn multiple_style_elements_preserve_document_order() {
    let style = element_style(
        "<style>.accent{color:red}</style><style>.accent{color:blue}</style>\
         <p class=\"accent\">Hello</p>",
        "p",
    );
    assert_eq!(style.color, Color::new(0, 0, 255, 255));
}

#[test]
fn style_rules_are_extracted_from_each_style_element_in_order() {
    let dom = parse_html("<style>p{color:red}</style><style>p{font-size:20px}</style><p>Hello</p>");
    let stylesheet = styles_from_dom(&dom);
    assert_eq!(stylesheet.rules.len(), 2);
    assert_eq!(stylesheet.rules[0].declarations[0].value, "red");
    assert_eq!(stylesheet.rules[1].declarations[0].value, "20px");
}
