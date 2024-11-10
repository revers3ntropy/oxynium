use crate::context::Context;
use crate::error::Error;
use crate::parse::token::Token;
use crate::types::Type;
use crate::util::MutRc;
use std::fmt;

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

    fn concrete(&self, ctx: MutRc<dyn Context>) -> Result<MutRc<dyn Type>, Error> {
        let key = self.identifier.clone().literal.unwrap().to_string();
        // all generics should be defined in a context, be it the concrete type,
        // or a generic placeholder.
        if !ctx.borrow().has_dec_with_id(&key.clone()) {
            println!("{}", ctx.borrow().str());
            println!("Failed to get dec with id: {}", key.clone());
        }
        let t = ctx.borrow().get_dec_from_id(&key.clone()).type_;
        if self.str() == "Q" {
            println!("{} >>> {:?}", self.str(), t.borrow().str());
        }
        Ok(t)
    }

    fn cache_id(&self, ctx: MutRc<dyn Context>) -> String {
        let self_id = &self.identifier.clone().literal.unwrap().to_string();

        let concrete_type = ctx.borrow().get_dec_from_id(self_id).type_;
        if format!("{:p}", concrete_type.as_ptr()) == format!("{:p}", self) {
            // avoid circular loop when the generic resolves to itself
            return format!("{}", self_id);
        }

        let concrete_type = concrete_type.borrow();
        concrete_type.cache_id(ctx.clone())
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
