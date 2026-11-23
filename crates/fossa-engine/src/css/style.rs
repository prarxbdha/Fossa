#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StyleSheet {
    pub rules: Vec<StyleRule>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StyleRule {
    pub selector: String,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Declaration {
    pub property: String,
    pub value: String,
}
