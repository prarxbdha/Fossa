use fossa_engine::css::selector::Selector;
use fossa_engine::dom::node::Node;

fn element(name: &str, attributes: &[(&str, &str)]) -> Node {
    Node::Element {
        name: name.to_owned(),
        attributes: attributes
            .iter()
            .map(|(key, value)| ((*key).to_owned(), (*value).to_owned()))
            .collect(),
        children: Vec::new(),
    }
}

#[test]
fn parses_only_supported_simple_selectors() {
    assert_eq!(Selector::parse("*"), Some(Selector::Universal));
    assert_eq!(Selector::parse(" P "), Some(Selector::Type("p".to_owned())));
    assert_eq!(
        Selector::parse(".accent"),
        Some(Selector::Class("accent".to_owned()))
    );
    assert_eq!(
        Selector::parse("#main"),
        Some(Selector::Id("main".to_owned()))
    );

    for unsupported in ["", "p:hover", ".a.b", "p > a", "p, div", "[href]", "*p"] {
        assert_eq!(Selector::parse(unsupported), None, "{unsupported}");
    }
}

#[test]
fn matches_universal_type_class_and_id_selectors() {
    let node = element("div", &[("class", "card accent"), ("id", "main")]);

    for source in ["*", "div", ".card", ".accent", "#main"] {
        let selector = Selector::parse(source).expect("supported selector");
        assert!(selector.matches(&node), "{source} should match");
    }
    for source in ["p", ".car", ".ac", "#card", "#Main"] {
        let selector = Selector::parse(source).expect("supported selector");
        assert!(!selector.matches(&node), "{source} should not match");
    }
}

#[test]
fn class_and_id_values_are_case_sensitive() {
    let node = element("p", &[("class", "Accent"), ("id", "Intro")]);
    assert!(!Selector::parse(".accent").unwrap().matches(&node));
    assert!(Selector::parse(".Accent").unwrap().matches(&node));
    assert!(!Selector::parse("#intro").unwrap().matches(&node));
    assert!(Selector::parse("#Intro").unwrap().matches(&node));
}

#[test]
fn text_and_document_nodes_do_not_match_selectors() {
    let text = Node::Text("hello".to_owned());
    assert!(!Selector::parse("*").unwrap().matches(&text));
    assert!(!Selector::parse("p").unwrap().matches(&Node::Document));
}

#[test]
fn specificity_orders_id_class_type_and_universal() {
    let specificity = |source: &str| Selector::parse(source).unwrap().specificity();
    assert!(specificity("#id") > specificity(".class"));
    assert!(specificity(".class") > specificity("p"));
    assert!(specificity("p") > specificity("*"));
}
