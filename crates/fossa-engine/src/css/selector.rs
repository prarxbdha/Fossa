use crate::dom::node::Node;

/// Specificity is ordered as (id, class, type/universal) counts.
pub type Specificity = (u32, u32, u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Selector {
    Universal,
    Type(String),
    Class(String),
    Id(String),
}

impl Selector {
    /// Parses only the four supported simple selectors. Anything else is
    /// rejected rather than approximated.
    pub fn parse(input: &str) -> Option<Self> {
        let trimmed = input.trim();
        if trimmed == "*" {
            return Some(Self::Universal);
        }
        if let Some(name) = trimmed.strip_prefix('.') {
            return is_simple_identifier(name).then(|| Self::Class(name.to_owned()));
        }
        if let Some(name) = trimmed.strip_prefix('#') {
            return is_simple_identifier(name).then(|| Self::Id(name.to_owned()));
        }
        if is_simple_identifier(trimmed) {
            return Some(Self::Type(trimmed.to_ascii_lowercase()));
        }
        None
    }

    pub fn matches(&self, node: &Node) -> bool {
        match node {
            Node::Element {
                name, attributes, ..
            } => self.matches_element(name, attributes),
            Node::Document | Node::Text(_) => false,
        }
    }

    pub fn matches_element(&self, name: &str, attributes: &[(String, String)]) -> bool {
        let attribute = |key: &str| {
            attributes
                .iter()
                .find(|(candidate, _)| candidate == key)
                .map(|(_, value)| value.as_str())
        };

        match self {
            Self::Universal => true,
            Self::Type(expected) => expected == name,
            Self::Class(expected) => attribute("class")
                .is_some_and(|classes| classes.split_whitespace().any(|class| class == expected)),
            Self::Id(expected) => attribute("id").is_some_and(|id| id == expected),
        }
    }

    pub fn specificity(&self) -> Specificity {
        match self {
            Self::Universal => (0, 0, 0),
            Self::Type(_) => (0, 0, 1),
            Self::Class(_) => (0, 1, 0),
            Self::Id(_) => (1, 0, 0),
        }
    }
}

fn is_simple_identifier(value: &str) -> bool {
    !value.is_empty()
        && value.chars().all(|character| {
            character.is_ascii_alphanumeric() || character == '-' || character == '_'
        })
}
