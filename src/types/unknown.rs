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
        _generics_map: HashMap<String, MutRc<dyn Type>>,
        _already_concrete: &mut HashMap<
            String,
            MutRc<dyn Type>,
        >,
    ) -> MutRc<dyn Type> {
        new_mut_rc(self.clone())
    }

    fn is_unknown(&self) -> bool {
        true
    }
}
