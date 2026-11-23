use fossa_engine::css::parser::parse;

#[test]
fn empty_css_produces_empty_stylesheet() {
    assert!(parse("").rules.is_empty());
}
