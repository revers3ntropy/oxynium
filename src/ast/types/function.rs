use std::fmt;
use std::rc::Rc;
use crate::ast::types::Type;

#[derive(Clone)]
pub struct FnType {
    pub name: String,
    pub ret_type: Rc<dyn Type>,
    pub parameters: Vec<Rc<dyn Type>>,
}

impl Type for FnType {
    fn is_ptr(&self) -> bool { true }
    fn str(&self) -> String {
        format!("Fn {}(): ", self.name)
    }

    fn contains(&self, t: Rc<dyn Type>) -> bool {
        if let Some(fn_type) = t.as_fn() {
            if fn_type.name != self.name {
                return false;
            }
            if fn_type.parameters.len() != self.parameters.len() {
                return false;
            }
            for i in 0..self.parameters.len() {
                if !self.parameters[i].contains(fn_type.parameters[i].clone()) {
                    return false;
                }
            }
            return true;
        }
        false
    }

    fn as_fn(&self) -> Option<FnType> {
        Some(self.clone())
    }
}

impl fmt::Debug for FnType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}>", self.str())
    }
}