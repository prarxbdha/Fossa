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
