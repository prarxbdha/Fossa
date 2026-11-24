use crate::css::style::{Declaration, StyleRule, StyleSheet};

/// Parses the deliberately small CSS subset used by Fossa: simple rule blocks
/// with `property: value;` declarations. Unsupported syntax is skipped without
/// discarding unrelated rules.
pub fn parse(css: &str) -> StyleSheet {
    let source = strip_comments(css);
    let mut rules = Vec::new();
    let mut position = 0usize;

    while position < source.len() {
        let Some(open) = source[position..].find('{').map(|offset| position + offset) else {
            break;
        };
        let selector = source[position..open].trim().trim_start_matches('}').trim();
        let Some(close) = find_block_end(&source, open) else {
            break;
        };

        if !selector.is_empty() {
            rules.push(StyleRule {
                selector: selector.to_owned(),
                declarations: parse_declarations(&source[open + 1..close]),
            });
        }
        position = close + 1;
    }

    StyleSheet { rules }
}

fn strip_comments(css: &str) -> String {
    let mut output = String::with_capacity(css.len());
    let mut index = 0usize;

    while index < css.len() {
        if css[index..].starts_with("/*") {
            match css[index + 2..].find("*/") {
                Some(end) => index += 2 + end + 2,
                None => break,
            }
        } else {
            let character = css[index..].chars().next().expect("index is inside input");
            output.push(character);
            index += character.len_utf8();
        }
    }
    output
}

fn find_block_end(source: &str, open: usize) -> Option<usize> {
    let mut depth = 0usize;
    for (offset, character) in source[open..].char_indices() {
        match character {
            '{' => depth += 1,
            '}' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some(open + offset);
                }
            }
            _ => {}
        }
    }
    None
}

fn parse_declarations(source: &str) -> Vec<Declaration> {
    source
        .split(';')
        .filter_map(|part| {
            let (property, value) = part.split_once(':')?;
            let property = property.trim();
            let value = value.trim();
            if property.is_empty() || value.is_empty() {
                return None;
            }
            Some(Declaration {
                property: property.to_ascii_lowercase(),
                value: value.to_owned(),
            })
        })
        .collect()
}
