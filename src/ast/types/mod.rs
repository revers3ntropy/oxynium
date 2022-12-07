use std::fmt::Debug;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Clone)]
pub struct Type {
    pub id: u64,
    pub name: String,
    // For atomic types, this is empty.
    // For types with parameters, this is a list of the parameters.
    // eg. for functions, the first element is the return type
    // and the rest are the parameter types.
    pub children: Vec<Rc<Type>>,
    pub is_ptr: bool,
}

impl Type {
    pub fn contains(&self, t: Rc<Type>) -> bool {
        if self.children.is_empty() {
            return self.id == t.id;
        }
        if t.children.len() != self.children.len() {
            return false;
        }
        for i in 0..self.children.iter().len() {
            if !self.children[i].contains(t.children[i].clone()) {
                return false;
            }
        }
        true
    }

    pub fn str(&self) -> String {
        if self.children.is_empty() {
            format!("{}", self.name)
        } else {
            format!("{}<{:?}>", self.name, self.children)
        }
    }
}

impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.str())
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.str())
    }
}