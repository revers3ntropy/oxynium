use std::fmt;
use std::rc::Rc;
use crate::ast::types::Type;

#[derive(Clone)]
pub struct AtomicType {
    pub id: u64,
    pub name: String,
    pub is_ptr: bool,
}

impl Type for AtomicType {
    fn is_ptr(&self) -> bool { self.is_ptr }
    fn str(&self) -> String {
        format!("{}", self.name)
    }

    fn contains(&self, t: Rc<dyn Type>) -> bool {
        if let Some(atomic_type) = t.as_atomic() {
            self.id == atomic_type.id
        } else {
            false
        }
    }

    fn as_atomic(&self) -> Option<AtomicType> {
        Some(self.clone())
    }
}

impl fmt::Debug for AtomicType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<Atomic {}>", self.str())
    }
}