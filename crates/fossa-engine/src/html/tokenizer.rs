#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    StartTag(String),
    EndTag(String),
    Text(String),
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut text = String::new();
    let mut index = 0usize;

    while index < input.len() {
        if input.as_bytes()[index] == b'<' {
            let rest = &input[index..];

            if rest.starts_with("<!--") {
                flush_text(&mut tokens, &mut text);
                match rest.find("-->") {
                    Some(end) => index += end + 3,
                    None => break,
                }
                continue;
            }

            if rest.starts_with("<!") || rest.starts_with("<?") {
                flush_text(&mut tokens, &mut text);
                match rest.find('>') {
                    Some(end) => index += end + 1,
                    None => break,
                }
                continue;
            }

            let end = match rest.find('>') {
                Some(end) => end,
                None => {
                    text.push_str(rest);
                    break;
                }
            };

            let raw = rest[1..end].trim();
            flush_text(&mut tokens, &mut text);

            if let Some(name) = raw.strip_prefix('/') {
                let name = tag_name(name);
                if !name.is_empty() {
                    tokens.push(Token::EndTag(name));
                }
            } else {
                let name = tag_name(raw);
                if !name.is_empty() {
                    tokens.push(Token::StartTag(name));
                }
            }

            index += end + 1;
        } else {
            let character = match input[index..].chars().next() {
                Some(character) => character,
                None => break,
            };
            text.push(character);
            index += character.len_utf8();
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

fn tag_name(raw: &str) -> String {
    raw.split(|character: char| character.is_ascii_whitespace() || character == '/')
        .next()
        .unwrap_or("")
        .trim_matches('<')
        .to_ascii_lowercase()
}

fn decode_entities(input: &str) -> String {
    input
        .replace("&nbsp;", " ")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
}
