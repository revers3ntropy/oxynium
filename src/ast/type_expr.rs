use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::{type_error, unknown_symbol, Error};
use crate::get_type;
use crate::parse::token::Token;
use crate::position::Interval;
use crate::types::unknown::UnknownType;
use crate::types::Type;
use crate::util::{new_mut_rc, MutRc};

#[derive(Debug, Clone)]
pub struct TypeNode {
    pub identifier: Token,
}

impl TypeNode {
    fn id(&self) -> String {
        self.identifier.literal.as_ref().unwrap().clone()
    }
}

impl Node for TypeNode {
    fn type_check(
        &self,
        ctx: MutRc<Context>,
    ) -> Result<TypeCheckRes, Error> {
        if !ctx.borrow_mut().has_dec_with_id(&self.id()) {
            if ctx.borrow().throw_on_unknowns() {
                return Err(unknown_symbol(format!(
                    "Type '{}'",
                    self.id()
                ))
                .set_interval(self.pos()));
            }
            return Ok(TypeCheckRes::unknown());
        }
        if !ctx.borrow().get_dec_from_id(&self.id()).is_type
        {
            return Err(type_error(format!(
                "'{}' cannot be used as a type",
                self.id()
            ))
            .set_interval(self.pos()));
        }

        let mut unknowns = 0;

        let type_: MutRc<dyn Type>;
        if !ctx.borrow().has_dec_with_id(&self.id()) {
            unknowns += 1;
            type_ = new_mut_rc(UnknownType {});
        } else {
            let t = get_type!(ctx, &self.id());

            let as_class = t.borrow().as_class();
            if let Some(class) = as_class {
                if class.generic_params_order.len() > 0 {
                    return Err(type_error(format!(
                        "Generics required for '{}'",
                        self.id()
                    ))
                    .set_interval(self.pos()));
                } else {
                    type_ = t;
                }
            } else {
                type_ = t;
            }
        }

        if type_.borrow().is_unknown() {
            unknowns += 1;
        }

        Ok(TypeCheckRes::from(type_, unknowns))
    }

    fn pos(&self) -> Interval {
        self.identifier.interval()
    }

    fn as_type_expr(&self) -> Option<TypeNode> {
        Some(self.clone())
    }
}
