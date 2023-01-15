use crate::context::Context;
use crate::error::{unknown_symbol, Error};
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
            || other.borrow().as_generic().is_some()
    }

    fn concrete(
        &self,
        ctx: MutRc<Context>,
        generics_map: HashMap<String, MutRc<dyn Type>>,
        _already_concrete: &mut HashMap<
            String,
            MutRc<dyn Type>,
        >,
    ) -> Result<MutRc<dyn Type>, Error> {
        let key = self
            .identifier
            .clone()
            .literal
            .unwrap()
            .to_string();
        if generics_map.contains_key(&key.clone()) {
            return Ok(generics_map
                .get(&key)
                .unwrap()
                .clone());
        }

        if ctx.borrow().has_dec_with_id(&key.clone()) {
            return Ok(ctx
                .borrow()
                .get_dec_from_id(&key.clone())
                .type_);
        }

        return Err(unknown_symbol(format!("{}", key))
            .set_interval(self.identifier.interval()));
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
