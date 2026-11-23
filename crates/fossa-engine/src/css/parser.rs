use crate::css::style::{Declaration, StyleRule, StyleSheet};

pub fn parse(css: &str) -> StyleSheet {
    let mut rules = Vec::new();

    for block in css.split('}') {
        let Some((selector, declarations)) = block.split_once('{') else {
            continue;
        };
        let selector = selector.trim();
        if selector.is_empty() {
            continue;
        }

        let declarations = declarations
            .split(';')
            .filter_map(|declaration| declaration.split_once(':'))
            .map(|(property, value)| Declaration {
                property: property.trim().to_owned(),
                value: value.trim().to_owned(),
            })
            .collect();

        rules.push(StyleRule {
            selector: selector.to_owned(),
            declarations,
        });
    }

    StyleSheet { rules }
}
