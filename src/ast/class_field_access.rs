use crate::ast::{AstNode, TypeCheckRes};
use crate::context::Context;
use crate::error::{type_error, Error};
use crate::parse::token::Token;
use crate::position::Interval;
use crate::types::Type;
use crate::util::{mut_rc, MutRc};

#[derive(Debug)]
pub struct FieldAccessNode {
    pub base: MutRc<dyn AstNode>,
    pub field_name: Token,
    pub position: Interval,
}

impl AstNode for FieldAccessNode {
    fn setup(&mut self, ctx: MutRc<dyn Context>) -> Result<(), Error> {
        self.base.borrow_mut().setup(ctx.clone())
    }
    fn type_check(&self, ctx: MutRc<dyn Context>) -> Result<TypeCheckRes, Error> {
        let TypeCheckRes {
            t: base_type_any,
            unknowns,
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
        let field_type = base_type.field_type(&key).or(base_type
            .method_type(&key)
            .map(|f| mut_rc(f.borrow().clone()) as MutRc<dyn Type>));

        if field_type.is_none() {
            return if ctx.borrow().throw_on_unknowns() {
                return Err(type_error(format!(
                    "Class '{}' does not have property '{}'",
                    base_type.str(),
                    self.field_name.clone().literal.unwrap(),
                ))
                .set_interval(self.position.clone()));
            } else {
                Ok(TypeCheckRes::unknown_and(unknowns))
            };
        }

        Ok(TypeCheckRes::from(field_type.unwrap(), unknowns))
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

        let key = self.field_name.clone().literal.unwrap();
        if base_type.field_type(&key).is_none() {
            let method = base_type.method_type(&key).unwrap();
            return Ok(format!(
                "
                    lea rax, [rel {}]
                    push rax
                ",
                method.borrow().name
            ));
        }
        let offset = base_type.field_offset(self.field_name.clone().literal.unwrap());
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
