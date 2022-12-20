use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::{type_error, Error};
use crate::parse::token::Token;
use crate::position::Interval;
use crate::types::Type;
use crate::util::MutRc;

#[derive(Debug)]
pub struct FieldAccessNode {
    pub base: MutRc<dyn Node>,
    pub field_name: Token,
    pub position: Interval,
}

impl Node for FieldAccessNode {
    fn asm(&mut self, ctx: MutRc<Context>) -> Result<String, Error> {
        let offset = self
            .base
            .borrow_mut()
            .type_check(ctx.clone())?
            .t
            .borrow_mut()
            .as_class()
            .unwrap()
            .field_offset(self.field_name.clone().literal.unwrap());

        Ok(format!(
            "
            {}
            pop rax
            push qword [rax + {offset}]
        ",
            self.base.borrow_mut().asm(ctx.clone())?,
        ))
    }

    fn type_check(&self, ctx: MutRc<Context>) -> Result<TypeCheckRes, Error> {
        let TypeCheckRes {
            t: base_type_any, ..
        } = self.base.borrow_mut().type_check(ctx.clone())?;
        let base_type = base_type_any.borrow().as_class();
        if base_type.is_none() {
            return Err(type_error(format!(
                "Cannot access field of non-class type '{}'",
                base_type_any.borrow().str()
            ))
            .set_interval(self.position.clone()));
        }
        let base_type = base_type.unwrap();

        let field_type =
            base_type.field_type(&self.field_name.clone().literal.unwrap());
        if field_type.is_none() {
            return Err(type_error(format!(
                "Class '{}' does not have field '{}'",
                base_type.str(),
                self.field_name.clone().literal.unwrap(),
            ))
            .set_interval(self.position.clone()));
        }
        Ok(TypeCheckRes::from(field_type.unwrap()))
    }

    fn pos(&self) -> Interval {
        self.position.clone()
    }
}
