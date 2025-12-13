use crate::error::Error;
use crate::parse::token::Token;
use crate::types::Type;
use crate::util::{mut_rc, MutRc};
use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;

#[derive(Clone)]
pub struct GenericType {
    pub identifier: Token,
}

impl fmt::Debug for GenericType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.identifier.clone().literal.unwrap())
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
        other.borrow().is_unknown() || other.borrow().as_generic().is_some()
    }

    fn concrete(
        &self,
        generics: &HashMap<String, MutRc<dyn Type>>,
        _cache: &mut HashMap<String, MutRc<dyn Type>>,
    ) -> Result<MutRc<dyn Type>, Error> {
        let key = self.identifier.clone().literal.unwrap().to_string();

        match generics.get(&key.clone()) {
            None => Ok(mut_rc(self.clone())),
            Some(v) => Ok(v.clone()),
        }
    }

    fn cache_id(&self, generics: &HashMap<String, MutRc<dyn Type>>) -> String {
        let self_id = &self.identifier.clone().literal.unwrap().to_string();
        match generics.get(self_id) {
            None => format!("{}", self.identifier.clone().literal.unwrap()),
            Some(concrete_type) => {
                let concrete_type = concrete_type.borrow();

                // SUPER HACKY!
                // I can't find a way to extract the raw pointer as a string :(

                // "Pointer { addr: 0xA, metadata: DynMetadata(0x0) }"
                let concrete_type_ptr_fmt = format!("{:p}", concrete_type.deref()).to_owned();
                // "0xA"
                let self_ptr_fmt = format!("{:p}", self);
                // where "0xA" will match if we are looking at ourselves here
                if concrete_type_ptr_fmt.contains(self_ptr_fmt.as_str()) {
                    // avoid circular loop when the generic is
                    // the same as the concrete type (not yet concreted)
                    format!("{}", self.identifier.clone().literal.unwrap())
                } else {
                    concrete_type.cache_id(generics)
                }
            }
        }
    }

    fn as_generic(&self) -> Option<GenericType> {
        Some(GenericType {
            identifier: self.identifier.clone(),
        })
    }
    fn is_unknown(&self) -> bool {
        false
    }
}
