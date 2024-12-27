use crate::ast::{AstNode, TypeCheckRes};
use crate::context::Context;
use crate::error::{type_error, Error};
use crate::parse::token::Token;
use crate::position::Interval;
use crate::types::Type;
use crate::util::MutRc;

#[derive(Debug)]
pub struct FieldAssignmentNode {
    pub base: MutRc<dyn AstNode>,
    pub field_name: Token,
    pub new_value: MutRc<dyn AstNode>,
    pub position: Interval,
}

impl AstNode for FieldAssignmentNode {
    fn setup(&mut self, ctx: MutRc<dyn Context>) -> Result<(), Error> {
        self.new_value.borrow_mut().setup(ctx.clone())?;
        self.base.borrow_mut().setup(ctx.clone())
    }
    fn type_check(&self, ctx: MutRc<dyn Context>) -> Result<TypeCheckRes, Error> {
        let TypeCheckRes {
            t: base_type_any,
            mut unknowns,
            ..
        } = self.base.borrow().type_check(ctx.clone())?;
        if base_type_any.borrow().is_unknown() {
            if ctx.borrow().throw_on_unknowns() {
                return Err(type_error(format!(
                    "cannot access field '{}' of unknown type",
                    self.field_name.literal.as_ref().unwrap()
                ))
                .set_interval(self.pos()));
            }
            return Ok(TypeCheckRes::unknown_and(unknowns));
        }

        let base_type = base_type_any.borrow().as_class().or(base_type_any
            .borrow()
            .as_type_type()
            .map(|t| t.instance_type.borrow().as_class().unwrap()));
        if base_type.is_none() {
            return Err(type_error(format!(
                "cannot access field `{}` of non-class type '{}'",
                self.field_name.literal.as_ref().unwrap(),
                base_type_any.borrow().str()
            ))
            .set_interval(self.position.clone()));
        }
        let base_type = base_type.unwrap();

        let key = self.field_name.clone().literal.unwrap();
        let field_type = base_type.field_type(&key);

        if field_type.is_none() {
            return if ctx.borrow().throw_on_unknowns() {
                return Err(type_error(format!(
                    "class '{}' does not have field '{}'",
                    base_type.str(),
                    self.field_name.clone().literal.unwrap(),
                ))
                .set_interval(self.position.clone()));
            } else {
                Ok(TypeCheckRes::unknown_and(unknowns))
            };
        }

        let TypeCheckRes {
            t: new_value_type,
            unknowns: new_value_unknowns,
            ..
        } = self.new_value.borrow().type_check(ctx.clone())?;
        unknowns += new_value_unknowns;

        if !field_type
            .clone()
            .unwrap()
            .borrow()
            .contains(new_value_type.clone())
        {
            return Err(type_error(format!(
                "expected type '{}', found '{}'",
                field_type.unwrap().borrow().str(),
                new_value_type.borrow().str()
            ))
            .set_interval(self.new_value.borrow().pos()));
        }
        if new_value_type.borrow().is_unknown() && ctx.borrow().throw_on_unknowns() {
            return Err(type_error(format!(
                "cannot assign unknown type to field '{}'",
                self.field_name.literal.as_ref().unwrap()
            ))
            .set_interval(self.new_value.borrow().pos()));
        }

        Ok(TypeCheckRes::from(new_value_type, unknowns))
    }

    fn asm(&mut self, ctx: MutRc<dyn Context>) -> Result<String, Error> {
        let base_type_any = self.base.borrow().type_check(ctx.clone())?.t;
        let base_type = base_type_any
            .borrow()
            .as_class()
            .or(base_type_any
                .borrow()
                .as_type_type()
                .map(|t| t.instance_type.borrow().as_class().unwrap()))
            .unwrap();

        let offset = base_type.field_offset(self.field_name.clone().literal.unwrap());
        Ok(format!(
            "
                {}
                {}
                pop rax
                pop rcx
                mov qword [rax + {offset}], rcx
                push rcx
            ",
            self.new_value.borrow_mut().asm(ctx.clone())?,
            self.base.borrow_mut().asm(ctx.clone())?,
        ))
    }

    fn pos(&self) -> Interval {
        self.position.clone()
    }
}
