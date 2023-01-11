use crate::parse::token::Token;
use crate::types::Type;
use crate::util::MutRc;
use std::fmt;

#[derive(Clone)]
pub struct TemplateType {
    pub identifier: Token,
}

impl fmt::Debug for TemplateType {
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

impl Type for TemplateType {
    fn is_ptr(&self) -> bool {
        true
    }

    fn str(&self) -> String {
        self.identifier.clone().literal.unwrap()
    }

    fn contains(&self, other: MutRc<dyn Type>) -> bool {
        if other.borrow().is_unknown() {
            return true;
        }
        // compare values of pointers...
        // TODO to this properly, with IDs or something
        format!("{:p}", self)
            == format!("{:p}", other.as_ptr())
    }

    fn is_unknown(&self) -> bool {
        false
    }
}
