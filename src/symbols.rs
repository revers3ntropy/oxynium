use crate::position::Interval;
use crate::types::Type;
use crate::util::MutRc;
use std::fmt::Debug;

const RESERVED_KEYWORDS: [&str; 50] = [
    "if",
    "else",
    "while",
    "for",
    "in",
    "break",
    "continue",
    "return",
    "let",
    "const",
    "mut",
    "var",
    "type",
    "fn",
    "extern",
    "primitive",
    "class",
    "struct",
    "enum",
    "trait",
    "interface",
    "use",
    "as",
    "mod",
    "import",
    "export",
    "is",
    "async",
    "await",
    "yield",
    "with",
    "unless",
    "pass",
    "case",
    "match",
    "comp",
    "del",
    "do",
    "new",
    "priv",
    "pub",
    "abstract",
    "try",
    "catch",
    "static",
    "except",
    "macro",
    "typeof",
    "true",
    "false",
];

pub fn is_valid_identifier(s: &str) -> bool {
    s.chars()
        .next()
        .map_or(false, |c| c.is_alphabetic() || c == '_')
        && s.chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '$')
        && !s.as_bytes()[0].is_ascii_digit()
        && !s.starts_with("_$")
}

pub fn can_declare_with_identifier(s: &str) -> bool {
    is_valid_identifier(s) && !RESERVED_KEYWORDS.contains(&s)
}

#[derive(Debug, Clone)]
pub struct SymbolDec {
    pub name: String,
    pub id: String,
    pub is_constant: bool,
    pub is_type: bool,
    pub type_: MutRc<dyn Type>,
    pub require_init: bool,
    pub is_defined: bool,
    pub is_param: bool,
    pub position: Interval,
}

impl SymbolDec {
    pub fn contains(&self, s: &SymbolDec) -> bool {
        self.type_.borrow().contains(s.type_.clone())
            && self.name == s.name
            && self.is_constant == s.is_constant
            && self.is_type == s.is_type
    }

    pub fn str(&self) -> String {
        format!("{}: {}", self.name, self.type_.borrow().str())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SymbolDef {
    pub name: String,
    pub data: Option<String>,
    pub text: Option<String>,
}
