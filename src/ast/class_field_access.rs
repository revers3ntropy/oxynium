use crate::ast::{AstNode, TypeCheckRes};
use crate::context::Context;
use crate::error::{type_error, Error};
use crate::parse::token::Token;
use crate::position::Interval;
use crate::types::Type;
use crate::util::MutRc;

#[derive(Debug)]
pub struct FieldAccessNode {
    pub base: MutRc<dyn AstNode>,
    pub field_name: Token,
    pub position: Interval,
}

impl AstNode for FieldAccessNode {
    fn setup(
        &mut self,
        ctx: MutRc<dyn Context>,
    ) -> Result<(), Error> {
        self.base.borrow_mut().setup(ctx.clone())
    }
    fn type_check(
        &self,
        ctx: MutRc<dyn Context>,
    ) -> Result<TypeCheckRes, Error> {
        let mut unknowns = 0;

        let TypeCheckRes {
            t: base_type_any,
            unknowns: base_unknowns,
            ..
        } = self.base.borrow().type_check(ctx.clone())?;
        if base_type_any.borrow().is_unknown() {
            if ctx.borrow().throw_on_unknowns() {
                return Err(type_error(format!(
                    "Cannot access field '{}' of unknown type",
                    self.field_name.literal.as_ref().unwrap()
                ))
                .set_interval(self.pos()));
            }
            return Ok(TypeCheckRes::unknown_and(
                base_unknowns,
            ));
        }
        unknowns += base_unknowns;

        let base_type = base_type_any.borrow().as_class();
        if base_type.is_none() {
            return Err(type_error(format!(
                "Cannot access field `{}` of non-class type '{}'",
                self.field_name.literal.as_ref().unwrap(),
                base_type_any.borrow().str()
            ))
            .set_interval(self.position.clone()));
        }
        let base_type = base_type.unwrap();

        let field_type = base_type.field_type(
            &self.field_name.clone().literal.unwrap(),
        );

        if field_type.is_none() {
            return if ctx.borrow().throw_on_unknowns() {
                return Err(type_error(format!(
                    "Class '{}' does not have field '{}'",
                    base_type.str(),
                    self.field_name
                        .clone()
                        .literal
                        .unwrap(),
                ))
                .set_interval(self.position.clone()));
            } else {
                Ok(TypeCheckRes::unknown_and(unknowns))
            };
        }

        Ok(TypeCheckRes::from(
            field_type.unwrap(),
            unknowns,
        ))
    }

    fn asm(
        &mut self,
        ctx: MutRc<dyn Context>,
    ) -> Result<String, Error> {
        let offset = self
            .base
            .borrow()
            .type_check(ctx.clone())?
            .t
            .borrow()
            .as_class()
            .unwrap()
            .field_offset(
                self.field_name.clone().literal.unwrap(),
            );

        Ok(format!(
            "
            {}
            pop rax
            push qword [rax + {offset}]
        ",
            self.base.borrow_mut().asm(ctx.clone())?,
        ))
    }

    fn pos(&self) -> Interval {
        self.position.clone()
    }
}
