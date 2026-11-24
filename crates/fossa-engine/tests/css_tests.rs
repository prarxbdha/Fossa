use fossa_engine::css::cascade::{CssValue, MAX_FONT_SIZE, parse_value};
use fossa_engine::css::parser::parse;
use fossa_engine::css::selector::Selector;
use fossa_engine::paint::display_list::Color;

#[test]
fn empty_css_produces_empty_stylesheet() {
    assert!(parse("").rules.is_empty());
}

#[test]
fn parses_multiple_rules_comments_and_missing_trailing_semicolon() {
    let sheet = parse("/* intro */ p { color: red; font-size: 16px } .accent { COLOR: blue; }");

    assert_eq!(sheet.rules.len(), 2);
    assert_eq!(sheet.rules[0].selector, "p");
    assert_eq!(sheet.rules[0].declarations[0].property, "color");
    assert_eq!(sheet.rules[0].declarations[0].value, "red");
    assert_eq!(sheet.rules[0].declarations[1].property, "font-size");
    assert_eq!(sheet.rules[0].declarations[1].value, "16px");
    assert_eq!(sheet.rules[1].selector, ".accent");
    assert_eq!(sheet.rules[1].declarations[0].property, "color");
}

#[test]
fn malformed_declarations_do_not_discard_valid_declarations() {
    let sheet = parse("p { color: red; ; broken; font-size: 18px; border: 1px solid; }");
    let declarations = &sheet.rules[0].declarations;

    assert_eq!(declarations.len(), 3);
    assert_eq!(
        declarations
            .iter()
            .map(|declaration| declaration.property.as_str())
            .collect::<Vec<_>>(),
        vec!["color", "font-size", "border"]
    );
    assert_eq!(
        declarations[2].value, "1px solid",
        "unsupported declarations remain raw but are ignored later"
    );
}

#[test]
fn malformed_rules_and_unsupported_selectors_do_not_match() {
    let sheet = parse("p:hover { color: red } } p { color: blue } .a.b { color: red }");

    assert_eq!(sheet.rules.len(), 3);
    assert!(Selector::parse(&sheet.rules[0].selector).is_none());
    assert_eq!(
        Selector::parse(&sheet.rules[1].selector),
        Some(Selector::Type("p".to_owned()))
    );
    assert!(Selector::parse(&sheet.rules[2].selector).is_none());
}

#[test]
fn hex_and_named_colors_resolve_to_rgba() {
    assert_eq!(
        parse_value("color", "#24c8db"),
        Some(CssValue::Color(Color::new(36, 200, 219, 255)))
    );
    assert_eq!(
        parse_value("color", "#fff"),
        Some(CssValue::Color(Color::new(255, 255, 255, 255)))
    );
    assert_eq!(
        parse_value("color", "TRANSPARENT"),
        Some(CssValue::Color(Color::new(0, 0, 0, 0)))
    );
    assert_eq!(parse_value("color", "#24xzdb"), None);
    assert_eq!(parse_value("color", "rgb(1, 2, 3)"), None);
}

#[test]
fn font_size_values_are_limited_and_validated() {
    assert_eq!(
        parse_value("font-size", "16px"),
        Some(CssValue::FontSize(16.0))
    );
    assert_eq!(
        parse_value("font-size", "18.5px"),
        Some(CssValue::FontSize(18.5))
    );
    assert_eq!(parse_value("font-size", "0"), Some(CssValue::FontSize(0.0)));
    assert_eq!(
        parse_value("font-size", "0px"),
        Some(CssValue::FontSize(0.0))
    );
    assert_eq!(parse_value("font-size", "-2px"), None);
    assert_eq!(parse_value("font-size", "1em"), None);
    assert_eq!(parse_value("font-size", "NaNpx"), None);
    assert_eq!(parse_value("font-size", "999999px"), None);
    assert_eq!(
        parse_value("font-size", "10000px"),
        Some(CssValue::FontSize(MAX_FONT_SIZE))
    );
}
