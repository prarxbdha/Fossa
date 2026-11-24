#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Document,
    Element {
        name: String,
        attributes: Vec<(String, String)>,
        children: Vec<Node>,
    },
    Text(String),
}

impl Node {
    /// Returns the lowercase tag name for element nodes.
    pub fn name(&self) -> Option<&str> {
        match self {
            Self::Element { name, .. } => Some(name),
            Self::Document | Self::Text(_) => None,
        }
    }

    /// Looks up an attribute by its normalized (lowercase) name.
    pub fn attribute(&self, name: &str) -> Option<&str> {
        match self {
            Self::Element { attributes, .. } => attributes
                .iter()
                .find(|(key, _)| key == name)
                .map(|(_, value)| value.as_str()),
            Self::Document | Self::Text(_) => None,
        }
    }
}
