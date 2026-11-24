use crate::css::selector::{Selector, Specificity};
use crate::css::style::StyleSheet;
use crate::dom::node::Node;
use crate::paint::display_list::{Color, DEFAULT_TEXT_COLOR};

/// Font sizes above this limit are rejected to keep layout geometry finite.
pub const MAX_FONT_SIZE: f32 = 10_000.0;
pub const DEFAULT_FONT_SIZE: f32 = 14.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComputedStyle {
    pub color: Color,
    pub font_size: f32,
}

impl Default for ComputedStyle {
    fn default() -> Self {
        Self {
            color: DEFAULT_TEXT_COLOR,
            font_size: DEFAULT_FONT_SIZE,
        }
    }
}

/// A parsed declaration value for one of the supported CSS properties.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CssValue {
    Color(Color),
    FontSize(f32),
}

/// A style-annotated view over the DOM. It is not a second DOM model; nodes
/// are borrowed from the parser's tree and carry only resolved style data.
#[derive(Debug, Clone)]
pub struct StyledNode<'a> {
    pub node: &'a Node,
    pub style: ComputedStyle,
    pub children: Vec<StyledNode<'a>>,
}

impl StyledNode<'_> {
    pub fn name(&self) -> Option<&str> {
        self.node.name()
    }
}

pub fn compute_styles<'a>(dom: &'a Node, stylesheet: &StyleSheet) -> StyledNode<'a> {
    build_styled_node(dom, stylesheet, ComputedStyle::default())
}

fn build_styled_node<'a>(
    node: &'a Node,
    stylesheet: &StyleSheet,
    inherited: ComputedStyle,
) -> StyledNode<'a> {
    let style = match node {
        Node::Element {
            name, attributes, ..
        } => resolve_element_style(name, attributes, stylesheet, inherited),
        Node::Document | Node::Text(_) => inherited,
    };

    let children = match node {
        Node::Element { children, .. } => children
            .iter()
            .map(|child| build_styled_node(child, stylesheet, style))
            .collect(),
        Node::Document | Node::Text(_) => Vec::new(),
    };

    StyledNode {
        node,
        style,
        children,
    }
}

/// Resolves one element's style: inherited values, built-in defaults, then
/// matching author declarations. Each property cascades independently.
pub fn resolve_element_style(
    name: &str,
    attributes: &[(String, String)],
    stylesheet: &StyleSheet,
    inherited: ComputedStyle,
) -> ComputedStyle {
    let mut style = ComputedStyle {
        color: inherited.color,
        font_size: default_font_size_for(name, inherited.font_size),
    };
    let mut best_color: Option<(Specificity, Color)> = None;
    let mut best_font_size: Option<(Specificity, f32)> = None;

    for rule in &stylesheet.rules {
        let Some(selector) = Selector::parse(&rule.selector) else {
            continue;
        };
        if !selector.matches_element(name, attributes) {
            continue;
        }
        let specificity = selector.specificity();

        for declaration in &rule.declarations {
            match parse_value(&declaration.property, &declaration.value) {
                Some(CssValue::Color(color))
                    if best_color.is_none_or(|(best, _)| specificity >= best) =>
                {
                    best_color = Some((specificity, color));
                }
                Some(CssValue::FontSize(size))
                    if best_font_size.is_none_or(|(best, _)| specificity >= best) =>
                {
                    best_font_size = Some((specificity, size));
                }
                Some(_) | None => {}
            }
        }
    }

    if let Some((_, color)) = best_color {
        style.color = color;
    }
    if let Some((_, font_size)) = best_font_size {
        style.font_size = font_size;
    }
    style
}

/// Built-in heading sizes are defaults only; author CSS overrides them.
pub fn default_font_size_for(name: &str, inherited: f32) -> f32 {
    match name {
        "h1" => 24.0,
        "h2" => 19.0,
        "h3" => 16.0,
        _ => safe_font_size(inherited),
    }
}

pub fn parse_value(property: &str, value: &str) -> Option<CssValue> {
    match property {
        "color" => parse_color(value).map(CssValue::Color),
        "font-size" => parse_font_size(value).map(CssValue::FontSize),
        _ => None,
    }
}

fn parse_color(value: &str) -> Option<Color> {
    let value = value.trim();
    let color = match value.to_ascii_lowercase().as_str() {
        "black" => Color::new(0, 0, 0, 255),
        "white" => Color::new(255, 255, 255, 255),
        "red" => Color::new(255, 0, 0, 255),
        "green" => Color::new(0, 128, 0, 255),
        "blue" => Color::new(0, 0, 255, 255),
        "gray" => Color::new(128, 128, 128, 255),
        "transparent" => Color::new(0, 0, 0, 0),
        other => parse_hex_color(other)?,
    };
    Some(color)
}

fn parse_hex_color(value: &str) -> Option<Color> {
    let hex = value.strip_prefix('#')?;
    match hex.len() {
        3 => {
            let r = hex_digit(hex.as_bytes()[0])?;
            let g = hex_digit(hex.as_bytes()[1])?;
            let b = hex_digit(hex.as_bytes()[2])?;
            Some(Color::new(r * 17, g * 17, b * 17, 255))
        }
        6 => {
            let bytes = hex.as_bytes();
            Some(Color::new(
                hex_digit(bytes[0])? * 16 + hex_digit(bytes[1])?,
                hex_digit(bytes[2])? * 16 + hex_digit(bytes[3])?,
                hex_digit(bytes[4])? * 16 + hex_digit(bytes[5])?,
                255,
            ))
        }
        _ => None,
    }
}

fn hex_digit(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

fn parse_font_size(value: &str) -> Option<f32> {
    let value = value.trim();
    if value == "0" {
        return Some(0.0);
    }
    let number = value.strip_suffix("px")?;
    if number.is_empty() || number.starts_with('-') {
        return None;
    }
    let size: f32 = number.parse().ok()?;
    if size.is_finite() && size >= 0.0 && size <= MAX_FONT_SIZE {
        Some(size)
    } else {
        None
    }
}

fn safe_font_size(value: f32) -> f32 {
    if value.is_finite() && value >= 0.0 {
        value.min(MAX_FONT_SIZE)
    } else {
        DEFAULT_FONT_SIZE
    }
}
