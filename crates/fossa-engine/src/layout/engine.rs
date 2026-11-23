use crate::dom::node::Node;
use crate::layout::box_tree::LayoutBox;

const SKIP_TAGS: &[&str] = &[
    "head", "title", "meta", "link", "style", "script", "base", "noscript",
];
const DEFAULT_FONT_SIZE: f32 = 14.0;
const CONTENT_PADDING: f32 = 24.0;
const CHAR_WIDTH_RATIO: f32 = 0.55;
const LINE_HEIGHT_RATIO: f32 = 1.4;
const BLOCK_GAP: f32 = 4.0;

/// Lays out a document into stacked block boxes. Text is greedily wrapped to
/// the available width using an approximate glyph width; no CSS is applied.
pub fn layout(node: &Node, viewport_width: f32) -> LayoutBox {
    let mut y = CONTENT_PADDING;
    let mut root = layout_node(node, viewport_width, DEFAULT_FONT_SIZE, &mut y);
    root.width = viewport_width;
    root
}

fn layout_node(node: &Node, width: f32, font_size: f32, y: &mut f32) -> LayoutBox {
    match node {
        Node::Document => LayoutBox {
            width,
            font_size,
            ..Default::default()
        },
        Node::Text(text) => {
            let content_width = (width - 2.0 * CONTENT_PADDING).max(font_size);
            let line_height = font_size * LINE_HEIGHT_RATIO;
            let start = *y;
            let children = wrap_text(text, content_width, font_size)
                .into_iter()
                .map(|line| {
                    let line_box = LayoutBox {
                        x: CONTENT_PADDING,
                        y: *y,
                        width: approximate_width(&line, font_size).min(content_width),
                        height: line_height,
                        text: Some(line),
                        font_size,
                        ..Default::default()
                    };
                    *y += line_height;
                    line_box
                })
                .collect();

            LayoutBox {
                width,
                height: *y - start,
                children,
                font_size,
                y: start,
                ..Default::default()
            }
        }
        Node::Element { name, children, .. } => {
            if SKIP_TAGS.contains(&name.as_str()) {
                return LayoutBox {
                    width,
                    y: *y,
                    font_size,
                    ..Default::default()
                };
            }

            let start = *y;
            let child_font_size = font_size_for(name, font_size);
            let box_children = layout_children(children, width, child_font_size, y);

            LayoutBox {
                width,
                height: *y - start,
                children: box_children,
                y: start,
                font_size: child_font_size,
                ..Default::default()
            }
        }
    }
}

fn layout_children(children: &[Node], width: f32, font_size: f32, y: &mut f32) -> Vec<LayoutBox> {
    let mut boxes = Vec::new();

    for child in children {
        let layout_box = layout_node(child, width, font_size, y);
        *y += layout_box.height + BLOCK_GAP;
        if layout_box.height > 0.0 {
            boxes.push(layout_box);
        }
    }

    boxes
}

fn font_size_for(name: &str, inherited: f32) -> f32 {
    match name {
        "h1" => 24.0,
        "h2" => 19.0,
        "h3" => 16.0,
        _ => inherited,
    }
}

fn approximate_width(text: &str, font_size: f32) -> f32 {
    text.chars().count() as f32 * font_size * CHAR_WIDTH_RATIO
}

/// Greedy word wrap using approximate glyph widths. Long unbroken words are
/// kept on one line; the canvas clips them at the viewport edge.
fn wrap_text(text: &str, max_width: f32, font_size: f32) -> Vec<String> {
    let words: Vec<&str> = text.split_whitespace().collect();
    if words.is_empty() {
        return Vec::new();
    }

    let max_chars = (max_width / (font_size * CHAR_WIDTH_RATIO)).max(1.0) as usize;
    let mut lines = Vec::new();
    let mut current = String::new();

    for word in words {
        if current.is_empty() {
            current.push_str(word);
        } else if current.chars().count() + 1 + word.chars().count() <= max_chars {
            current.push(' ');
            current.push_str(word);
        } else {
            lines.push(std::mem::take(&mut current));
            current.push_str(word);
        }
    }

    if !current.is_empty() {
        lines.push(current);
    }

    lines
}
