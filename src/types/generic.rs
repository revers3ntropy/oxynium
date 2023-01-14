use crate::parse::token::Token;
use crate::types::Type;
use crate::util::MutRc;
use std::collections::HashMap;
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
            || format!("{:p}", self)
                == format!("{:p}", other.as_ptr())
    }

    fn concrete(
        &self,
        generics_map: HashMap<String, MutRc<dyn Type>>,
        _already_concrete: &mut HashMap<
            String,
            MutRc<dyn Type>,
        >,
    ) -> MutRc<dyn Type> {
        generics_map
            .get(
                &self
                    .identifier
                    .clone()
                    .literal
                    .unwrap()
                    .to_string(),
            )
            .unwrap()
            .clone()
        //.borrow()
        //.concrete(generics_map, already_concrete)
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
