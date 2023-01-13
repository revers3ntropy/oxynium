use crate::parse::token::Token;
use crate::types::Type;
use crate::util::MutRc;
use std::fmt;

#[derive(Clone)]
pub struct GenericType {
    pub identifier: Token,
}

impl fmt::Debug for GenericType {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            f,
            "{}",
            self.identifier.clone().literal.unwrap()
        )
    }
}

impl Type for GenericType {
    fn is_ptr(&self) -> bool {
        true
    }

    fn str(&self) -> String {
        self.identifier.clone().literal.unwrap()
    }

    fn contains(&self, other: MutRc<dyn Type>) -> bool {
        other.borrow().is_unknown()
    }

    fn is_unknown(&self) -> bool {
        false
    }
    fn as_generic(&self) -> Option<GenericType> {
        Some(GenericType {
            identifier: self.identifier.clone(),
        })
    }
}
