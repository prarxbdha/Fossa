#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Ident(String),
    String(String),
    Symbol(char),
}
