use crate::context::Context;
use crate::error::Error;
use crate::types::Type;
use crate::util::{new_mut_rc, MutRc};
use std::fmt;

#[derive(Clone)]
pub struct TypeType {
    pub instance_type: MutRc<dyn Type>,
}

impl Type for TypeType {
    fn is_ptr(&self) -> bool {
        true
    }

    fn str(&self) -> String {
        self.instance_type.borrow().str()
    }

    fn contains(&self, other: MutRc<dyn Type>) -> bool {
        if other.borrow().is_unknown() {
            return true;
        }
        if let Some(other) = other.borrow().as_type_type() {
            self.instance_type
                .borrow()
                .contains(other.instance_type.clone())
        } else {
            false
        }
    }

    fn concrete(
        &self,
        ctx: MutRc<dyn Context>,
    ) -> Result<MutRc<dyn Type>, Error> {
        Ok(new_mut_rc(TypeType {
            instance_type: self
                .instance_type
                .borrow()
                .concrete(ctx)?,
        }))
    }

    fn cache_id(&self, ctx: MutRc<dyn Context>) -> String {
        format!(
            "*{}",
            self.instance_type
                .borrow()
                .cache_id(ctx.clone())
        )
    }

    fn as_type_type(&self) -> Option<TypeType> {
        Some(self.clone())
    }
}

impl fmt::Debug for TypeType {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "Type<{:?}>", self.instance_type)
    }
}
