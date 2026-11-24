use crate::css::cascade::{DEFAULT_FONT_SIZE, MAX_FONT_SIZE, StyledNode};
use crate::layout::box_tree::LayoutBox;

const SKIP_TAGS: &[&str] = &[
    "head", "title", "meta", "link", "style", "script", "base", "noscript",
];
const CONTENT_PADDING: f32 = 24.0;
const CHAR_WIDTH_RATIO: f32 = 0.55;
const LINE_HEIGHT_RATIO: f32 = 1.4;
const BLOCK_GAP: f32 = 4.0;

/// Lays out a styled document into stacked block boxes. Text is greedily
/// wrapped to the available width using an approximate glyph width.
pub fn layout(root: &StyledNode<'_>, viewport_width: f32) -> LayoutBox {
    let width = if viewport_width.is_finite() && viewport_width >= 0.0 {
        viewport_width
    } else {
        0.0
    };
    let mut y = CONTENT_PADDING;
    let mut layout_root = layout_node(root, width, &mut y);
    layout_root.width = width;
    layout_root
}

fn layout_node(styled: &StyledNode<'_>, width: f32, y: &mut f32) -> LayoutBox {
    let font_size = safe_font_size(styled.style.font_size);
    let color = styled.style.color;

    match styled.node {
        crate::dom::node::Node::Document => LayoutBox {
            width,
            font_size,
            color,
            ..Default::default()
        },
        crate::dom::node::Node::Text(text) => {
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
                        color,
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
                color,
                y: start,
                ..Default::default()
            }
        }
        crate::dom::node::Node::Element { name, .. } => {
            if SKIP_TAGS.contains(&name.as_str()) {
                return LayoutBox {
                    width,
                    y: *y,
                    font_size,
                    color,
                    ..Default::default()
                };
            }

            let start = *y;
            let box_children = layout_children(&styled.children, width, y);

            LayoutBox {
                width,
                height: *y - start,
                children: box_children,
                y: start,
                font_size,
                color,
                ..Default::default()
            }
        }
    }
}

fn layout_children(children: &[StyledNode<'_>], width: f32, y: &mut f32) -> Vec<LayoutBox> {
    let mut boxes = Vec::new();

    for child in children {
        let layout_box = layout_node(child, width, y);
        *y += layout_box.height + BLOCK_GAP;
        if layout_box.height > 0.0 {
            boxes.push(layout_box);
        }
    }

    boxes
}

fn safe_font_size(value: f32) -> f32 {
    if value.is_finite() && value >= 0.0 {
        value.min(MAX_FONT_SIZE)
    } else {
        DEFAULT_FONT_SIZE
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

    let max_chars = if font_size <= 0.0 {
        usize::MAX
    } else {
        (max_width / (font_size * CHAR_WIDTH_RATIO)).max(1.0) as usize
    };
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
