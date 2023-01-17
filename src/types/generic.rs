use crate::context::Context;
use crate::error::{unknown_symbol, Error};
use crate::parse::token::Token;
use crate::types::unknown::UnknownType;
use crate::types::Type;
use crate::util::{new_mut_rc, MutRc};
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
    ) -> Result<MutRc<dyn Type>, Error> {
        let key = self
            .identifier
            .clone()
            .literal
            .unwrap()
            .to_string();

        if ctx.borrow().has_dec_with_id(&key.clone()) {
            let t = ctx
                .borrow()
                .get_dec_from_id(&key.clone())
                .type_;
            return Ok(t);
        }
        if ctx.borrow().throw_on_unknowns() {
            return Err(unknown_symbol(format!(
                "generic '{}'",
                key
            ))
            .set_interval(self.identifier.interval()));
        }
        Ok(new_mut_rc(UnknownType {}))
    }

    fn cache_id(&self, ctx: MutRc<Context>) -> String {
        let concrete_type = ctx
            .borrow()
            .get_dec_from_id(
                &self
                    .identifier
                    .clone()
                    .literal
                    .unwrap()
                    .to_string(),
            )
            .type_;
        if format!("{:p}", concrete_type.as_ptr())
            == format!("{:p}", self)
        {
            // avoid circular loop when the generic is
            // the same as the concrete type (not yet concreted)

            // TODO: Why does this throw on valid code?
            // if ctx.borrow().throw_on_unknowns() {
            //     panic!("circular loop in generic type");
            // }

            format!(
                "{}",
                self.identifier.clone().literal.unwrap()
            )
        } else {
            let concrete_type = concrete_type.borrow();
            concrete_type.cache_id(ctx.clone())
        }
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
