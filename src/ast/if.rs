use crate::ast::{AstNode, TypeCheckRes};
use crate::context::Context;
use crate::error::{type_error, Error};
use crate::get_type;
use crate::position::Interval;
use crate::types::unknown::UnknownType;
use crate::util::{mut_rc, MutRc};

#[derive(Debug)]
pub struct IfNode {
    pub comparison: MutRc<dyn AstNode>,
    pub body: MutRc<dyn AstNode>,
    pub else_body: Option<MutRc<dyn AstNode>>,
    pub position: Interval,
}

impl AstNode for IfNode {
    fn setup(&mut self, ctx: MutRc<dyn Context>) -> Result<(), Error> {
        self.body.borrow_mut().setup(ctx.clone())?;
        if let Some(ref else_body) = self.else_body.clone() {
            else_body.borrow_mut().setup(ctx.clone())?;
        }
        self.comparison.borrow_mut().setup(ctx.clone())
    }
    fn type_check(&self, ctx: MutRc<dyn Context>) -> Result<TypeCheckRes, Error> {
        let TypeCheckRes {
            t: mut body_ret_type,
            is_returned: mut body_is_returned,
            mut always_returns,
            mut unknowns,
            ..
        } = self.body.borrow().type_check(ctx.clone())?;

        let TypeCheckRes {
            t: comp_type,
            unknowns: comp_unknowns,
            ..
        } = self.comparison.borrow().type_check(ctx.clone())?;
        unknowns += comp_unknowns;

        if !get_type!(ctx, "Bool").borrow().contains(comp_type) {
            return Err(type_error("if condition must be of type Bool".to_string())
                .set_interval(self.comparison.borrow().pos()));
        }

        if self.else_body.is_some() {
            let TypeCheckRes {
                t: else_ret_type,
                is_returned: else_is_returned,
                always_returns: else_always_returns,
                unknowns: else_unknowns,
                ..
            } = self
                .else_body
                .clone()
                .unwrap()
                .borrow_mut()
                .type_check(ctx.clone())?;
            unknowns += else_unknowns;

            always_returns = always_returns && else_always_returns;

            if body_is_returned {
                if else_is_returned {
                    if !body_ret_type.borrow().contains(else_ret_type.clone()) {
                        return Err(type_error(format!(
                            "if statement branches cannot return different types: {} and {}",
                            body_ret_type.borrow().str(),
                            else_ret_type.borrow().str()
                        )));
                    }
                }
            } else if else_is_returned {
                body_ret_type = else_ret_type;
                body_is_returned = true;
            }
        }
        if body_is_returned {
            Ok(TypeCheckRes::returns(
                always_returns && self.else_body.is_some(),
                body_ret_type,
                unknowns,
            ))
        } else {
            Ok(TypeCheckRes::from_type_in_ctx(&ctx, "Void", unknowns, true))
        }
    }

    fn asm(&mut self, ctx: MutRc<dyn Context>) -> Result<String, Error> {
        let body = self.body.borrow_mut().asm(ctx.clone())?;
        let comp = self.comparison.borrow_mut().asm(ctx.clone())?;
        let after_lbl = ctx.borrow_mut().get_anon_label();

        if self.else_body.is_some() {
            let else_body = self
                .else_body
                .take()
                .unwrap()
                .borrow_mut()
                .asm(ctx.clone())?;
            let else_lbl = ctx.borrow_mut().get_anon_label();

            Ok(format!(
                "
                    {comp}
                    pop rax
                    test rax, rax     ; if evaluates to false, don't do body
                    je {else_lbl}
                    {body}
                    jmp {after_lbl}
                    {else_lbl}:
                    {else_body}
                    {after_lbl}:
                "
            ))
        } else {
            Ok(format!(
                "
                    {comp}
                    pop rax
                    test rax, rax     ; if evaluates to false, don't do body
                    je {after_lbl}
                    {body}
                    {after_lbl}:
                "
            ))
        }
    }

    fn pos(&self) -> Interval {
        self.position.clone()
    }
}
