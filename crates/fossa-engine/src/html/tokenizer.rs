#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    StartTag {
        name: String,
        attributes: Vec<Attribute>,
        self_closing: bool,
    },
    EndTag(String),
    Text(String),
    /// Raw contents of `<style>` and `<script>`, emitted without entity decoding.
    RawText(String),
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut text = String::new();
    let mut index = 0usize;

    while index < input.len() {
        if input.as_bytes()[index] != b'<' {
            let character = input[index..]
                .chars()
                .next()
                .expect("index is inside input");
            text.push(character);
            index += character.len_utf8();
            continue;
        }

        let rest = &input[index..];

        if rest.starts_with("<!--") {
            flush_text(&mut tokens, &mut text);
            match rest.find("-->") {
                Some(end) => index += end + 3,
                None => {
                    text.push_str(rest);
                    break;
                }
            }
            continue;
        }

        if rest.starts_with("<!") || rest.starts_with("<?") {
            flush_text(&mut tokens, &mut text);
            match rest.find('>') {
                Some(end) => index += end + 1,
                None => {
                    text.push_str(rest);
                    break;
                }
            }
            continue;
        }

        let Some(end) = scan_tag_end(rest) else {
            text.push_str(rest);
            break;
        };
        flush_text(&mut tokens, &mut text);

        let tag = emit_tag(&rest[1..end], &mut tokens);
        index += end + 1;

        if matches!(tag.as_deref(), Some("style") | Some("script")) {
            consume_raw_text(
                &input[index..],
                tag.as_deref().unwrap(),
                &mut tokens,
                &mut index,
            );
        }
    }

    flush_text(&mut tokens, &mut text);
    tokens
}

fn flush_text(tokens: &mut Vec<Token>, text: &mut String) {
    if !text.is_empty() {
        tokens.push(Token::Text(decode_entities(text)));
        text.clear();
    }
}

/// Finds `>` while ignoring quoted attribute values, so a quoted `>` does not
/// terminate a tag early.
fn scan_tag_end(input: &str) -> Option<usize> {
    let mut quote = None;
    for (offset, character) in input.char_indices().skip(1) {
        match quote {
            Some(active) if character == active => quote = None,
            Some(_) => {}
            None if character == '\'' || character == '"' => quote = Some(character),
            None if character == '>' => return Some(offset),
            None => {}
        }
    }
    None
}

fn emit_tag(raw: &str, tokens: &mut Vec<Token>) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() || trimmed.starts_with('!') || trimmed.starts_with('?') {
        return None;
    }

    if let Some(rest) = trimmed.strip_prefix('/') {
        let name = tag_name(rest);
        if name.is_empty() {
            return None;
        }
        tokens.push(Token::EndTag(name.clone()));
        return Some(name);
    }

    let (name, attributes, self_closing) = parse_start_tag(trimmed);
    if name.is_empty() {
        return None;
    }
    tokens.push(Token::StartTag {
        name: name.clone(),
        attributes,
        self_closing,
    });
    Some(name)
}

fn parse_start_tag(raw: &str) -> (String, Vec<Attribute>, bool) {
    let trimmed = raw.trim_end();
    let self_closing = trimmed.ends_with('/');
    let source = if self_closing {
        trimmed.trim_end_matches('/').trim_end()
    } else {
        trimmed
    };
    let name_end = source
        .find(|character: char| character.is_ascii_whitespace() || character == '/')
        .unwrap_or(source.len());
    let name = source[..name_end].to_ascii_lowercase();
    let attributes = parse_attributes(&source[name_end..]);
    (name, attributes, self_closing)
}

fn parse_attributes(source: &str) -> Vec<Attribute> {
    let mut attributes: Vec<Attribute> = Vec::new();
    let mut index = 0usize;

    while index < source.len() {
        index = skip_whitespace(source, index);
        if index >= source.len() {
            break;
        }
        if source.as_bytes()[index] == b'/' {
            index += 1;
            continue;
        }

        let name_start = index;
        while index < source.len() {
            let byte = source.as_bytes()[index];
            if byte.is_ascii_whitespace() || byte == b'=' || byte == b'/' {
                break;
            }
            index += source[index..]
                .chars()
                .next()
                .map(char::len_utf8)
                .unwrap_or(1);
        }
        if index == name_start {
            index += source[index..]
                .chars()
                .next()
                .map(char::len_utf8)
                .unwrap_or(1);
            continue;
        }

        let name = source[name_start..index].to_ascii_lowercase();
        let after_name = skip_whitespace(source, index);
        let (value, next_index) =
            if after_name < source.len() && source.as_bytes()[after_name] == b'=' {
                parse_attribute_value(source, skip_whitespace(source, after_name + 1))
            } else {
                (String::new(), after_name)
            };
        index = next_index;

        if !attributes.iter().any(|attribute| attribute.name == name) {
            attributes.push(Attribute { name, value });
        }
    }
    attributes
}

fn parse_attribute_value(source: &str, mut index: usize) -> (String, usize) {
    if index >= source.len() {
        return (String::new(), index);
    }

    let quote = source.as_bytes()[index];
    if quote == b'"' || quote == b'\'' {
        index += 1;
        let start = index;
        while index < source.len() && source.as_bytes()[index] != quote {
            index += source[index..]
                .chars()
                .next()
                .map(char::len_utf8)
                .unwrap_or(1);
        }
        let value = decode_entities(&source[start..index]);
        if index < source.len() {
            index += 1;
        }
        return (value, index);
    }

    let start = index;
    while index < source.len() && !source.as_bytes()[index].is_ascii_whitespace() {
        index += source[index..]
            .chars()
            .next()
            .map(char::len_utf8)
            .unwrap_or(1);
    }
    (decode_entities(&source[start..index]), index)
}

fn skip_whitespace(input: &str, mut index: usize) -> usize {
    while index < input.len() {
        let character = input[index..]
            .chars()
            .next()
            .expect("index is inside input");
        if !character.is_ascii_whitespace() {
            break;
        }
        index += character.len_utf8();
    }
    index
}

fn consume_raw_text(remaining: &str, tag: &str, tokens: &mut Vec<Token>, index: &mut usize) {
    let lowered = remaining.to_ascii_lowercase();
    let needle = format!("</{tag}");
    let mut search_from = 0usize;

    while let Some(found) = lowered[search_from..].find(&needle) {
        let start = search_from + found;
        let after_name = start + needle.len();
        let boundary_is_valid = lowered
            .as_bytes()
            .get(after_name)
            .is_none_or(|byte| byte.is_ascii_whitespace() || *byte == b'>');
        if !boundary_is_valid {
            search_from = after_name;
            continue;
        }

        let Some(greater_than) = remaining[after_name..].find('>') else {
            break;
        };
        let close_end = after_name + greater_than + 1;
        if start > 0 {
            tokens.push(Token::RawText(remaining[..start].to_owned()));
        }
        emit_tag(&remaining[start + 1..close_end - 1], tokens);
        *index += close_end;
        return;
    }

    if !remaining.is_empty() {
        tokens.push(Token::RawText(remaining.to_owned()));
    }
    tokens.push(Token::EndTag(tag.to_owned()));
    *index += remaining.len();
}

fn tag_name(raw: &str) -> String {
    raw.split(|character: char| character.is_ascii_whitespace() || character == '/')
        .next()
        .unwrap_or("")
        .trim_matches('<')
        .to_ascii_lowercase()
}

/// Decodes the small entity set Fossa supports in a single pass, avoiding
/// accidental double-decoding such as `&amp;lt;`.
fn decode_entities(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut index = 0usize;

    while index < input.len() {
        if input.as_bytes()[index] == b'&' {
            if let Some(offset) = input[index + 1..].find(';') {
                let end = index + 1 + offset;
                if let Some(decoded) = decode_entity(&input[index + 1..end]) {
                    output.push(decoded);
                    index = end + 1;
                    continue;
                }
            }
        }
        let character = input[index..]
            .chars()
            .next()
            .expect("index is inside input");
        output.push(character);
        index += character.len_utf8();
    }
    output
}

fn decode_entity(entity: &str) -> Option<char> {
    match entity {
        "nbsp" => Some('\u{00a0}'),
        "amp" => Some('&'),
        "lt" => Some('<'),
        "gt" => Some('>'),
        "quot" => Some('"'),
        "apos" | "#39" => Some('\''),
        _ => decode_numeric_entity(entity),
    }
}

fn decode_numeric_entity(entity: &str) -> Option<char> {
    let digits = entity.strip_prefix('#')?;
    let (radix, digits) = match digits
        .strip_prefix('x')
        .or_else(|| digits.strip_prefix('X'))
    {
        Some(hex) => (16, hex),
        None => (10, digits),
    };
    let code = u32::from_str_radix(digits, radix).ok()?;
    char::from_u32(code)
}
