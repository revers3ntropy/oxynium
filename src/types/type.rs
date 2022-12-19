use crate::types::Type;
use crate::util::MutRc;
use std::fmt;

#[derive(Clone)]
pub struct TypeType {
    pub(crate) instance_type: MutRc<dyn Type>,
}

impl Type for TypeType {
    fn is_ptr(&self) -> bool {
        true
    }

    fn str(&self) -> String {
        self.instance_type.borrow().str()
    }

    fn contains(&self, other: MutRc<dyn Type>) -> bool {
        if let Some(other) = other.borrow().as_type_type() {
            self.instance_type
                .borrow()
                .contains(other.instance_type.clone())
        } else {
            false
        }
    }

    fn as_type_type(&self) -> Option<TypeType> {
        Some(self.clone())
    }
}

impl fmt::Debug for TypeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Type<{:?}>", self.instance_type)
    }
}
