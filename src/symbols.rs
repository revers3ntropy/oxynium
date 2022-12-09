use std::rc::Rc;
use crate::ast::types::Type;

const RESERVED_KEYWORDS: [&str; 15] = [
    "if",
    "else",
    "while",
    "for",
    "break",
    "continue",
    "return",
    "let",
    "const",
    "type",
    "let",
    "mut",
    "var",
    "fn",
    "extern"
];

pub fn is_valid_identifier(s: &str) -> bool {
    s.chars().next().map_or(false, |c| c.is_alphabetic() || c == '_')
        && s.chars().all(|c| c.is_alphanumeric() || c == '_')
        && RESERVED_KEYWORDS.contains(&s) == false
        && !s.as_bytes()[0].is_ascii_digit()
        && !s.starts_with("_$_")
}

#[derive(Debug, Clone)]
pub struct SymbolDec {
    pub name: String,
    pub id: String,
    pub is_constant: bool,
    pub is_type: bool,
    pub type_: Rc<Type>,
    pub require_init: bool,
    pub is_defined: bool,
}

impl SymbolDec {
    pub fn contains(&self, s: &SymbolDec) -> bool {
        self.type_.contains(s.type_.clone())
            && self.name == s.name
            && self.is_constant == s.is_constant
            && self.is_type == s.is_type
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SymbolDef {
    pub name: String,
    pub data: Option<String>,
    pub text: Option<String>,
    pub is_local: bool,
}