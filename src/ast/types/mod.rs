use std::fmt::Debug;
use std::fmt::Display;

pub mod built_in;

#[derive(Clone)]
pub struct Type {
    pub id: u64,
    pub name: &'static str,
    // For atomic types, this is empty.
    // For types with parameters, this is a list of the parameters.
    // eg. for functions, the first element is the return type
    // and the rest are the parameter types.
    pub children: Vec<Box<Type>>
}

impl Type {
    pub fn contains(&self, t: &Type) -> bool {
        if self.children.is_empty() {
            return self.id == t.id;
        }
        for child in self.children.iter() {
            if !child.contains(t) {
                return false;
            }
        }
        true
    }
}

impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}