use crate::parse::token::Token;
use crate::types::Type;
use crate::util::{new_mut_rc, MutRc};
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

    fn operator_signature(
        &self,
        _op: Token,
    ) -> Option<MutRc<dyn Type>> {
        Some(new_mut_rc(UnknownType {}))
    }

    fn contains(&self, _: MutRc<dyn Type>) -> bool {
        true
    }

    fn is_unknown(&self) -> bool {
        true
    }
}
