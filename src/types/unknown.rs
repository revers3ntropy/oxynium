use crate::types::Type;
use crate::util::MutRc;
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

    fn is_unknown(&self) -> bool {
        true
    }
}
