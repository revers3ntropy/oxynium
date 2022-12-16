use crate::ast::types::Type;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

#[derive(Clone)]
pub struct InstanceType {
    pub struct_type: Rc<dyn Type>,
}

impl Debug for InstanceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.str())
    }
}

impl Type for InstanceType {
    fn is_ptr(&self) -> bool {
        true
    }
    fn str(&self) -> String {
        format!("Instance of {}", self.struct_type.str())
    }

    fn contains(&self, t: Rc<dyn Type>) -> bool {
        std::ptr::eq(t.as_ref(), self)
    }

    fn as_instance(&self) -> Option<InstanceType> {
        Some(self.clone())
    }
}
