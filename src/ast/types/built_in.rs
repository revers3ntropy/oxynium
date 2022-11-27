use crate::ast::types::Type;

pub const BOOL: Type = Type {
    id: 0,
    name: "bool",
    children: vec![]
};
pub const INT: Type = Type {
    id: 1,
    name: "int",
    children: vec![]
};
pub const STR: Type = Type {
    id: 2,
    name: "str",
    children: vec![]
};
pub const VOID: Type = Type {
    id: 3,
    name: "void",
    children: vec![]
};