use crate::context::Context;
use crate::error::Error;
use crate::types::Type;
use crate::util::{new_mut_rc, MutRc};
use std::collections::HashMap;
use std::fmt;

#[derive(Clone)]
pub struct UnknownType;

impl fmt::Debug for UnknownType {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "?")
    }
}

impl Type for UnknownType {
    fn is_ptr(&self) -> bool {
        true
    }

    fn str(&self) -> String {
        "?".to_string()
    }

    fn contains(&self, _: MutRc<dyn Type>) -> bool {
        true
    }

    fn concrete(
        &self,
        _ctx: MutRc<Context>,
        _generic_args: MutRc<
            HashMap<String, MutRc<dyn Type>>,
        >,
    ) -> Result<MutRc<dyn Type>, Error> {
        Ok(new_mut_rc(self.clone()))
    }

    fn is_unknown(&self) -> bool {
        true
    }
}
