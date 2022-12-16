use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::{type_error, Error};
use crate::position::Interval;
use crate::util::MutRc;

#[derive(Debug)]
pub struct IfNode {
    pub comparison: MutRc<dyn Node>,
    pub body: MutRc<dyn Node>,
    pub else_body: Option<MutRc<dyn Node>>,
    pub position: Interval,
}

impl Node for IfNode {
    fn asm(&mut self, ctx: MutRc<Context>) -> Result<String, Error> {
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

    fn type_check(
        &mut self,
        ctx: MutRc<Context>,
    ) -> Result<TypeCheckRes, Error> {
        let (_, mut body_ret_type) =
            self.body.borrow_mut().type_check(ctx.clone())?;

        let (comp_type, _) =
            self.comparison.borrow_mut().type_check(ctx.clone())?;
        if !ctx
            .borrow_mut()
            .get_dec_from_id("Bool")?
            .type_
            .contains(comp_type)
        {
            return Err(type_error("if condition must be a bool".to_string())
                .set_interval(self.comparison.borrow_mut().pos()));
        }

        if self.else_body.is_some() {
            let else_body = self.else_body.take().unwrap();
            let (_, else_ret_type) =
                else_body.borrow_mut().type_check(ctx.clone())?;
            self.else_body = Some(else_body);

            if let Some(body_ret) = body_ret_type.take() {
                if let Some(else_ret_type) = else_ret_type {
                    if !body_ret.contains(else_ret_type.clone()) {
                        return Err(type_error(format!(
                            "if statement branches cannot return different types: {} and {}",
                            body_ret.str(),
                            else_ret_type.str()
                        )));
                    }
                }
                body_ret_type = Some(body_ret);
            } else if else_ret_type.is_some() {
                body_ret_type = else_ret_type;
            }
        }
        Ok((
            ctx.borrow_mut().get_dec_from_id("Void")?.type_.clone(),
            body_ret_type,
        ))
    }

    fn pos(&mut self) -> Interval {
        self.position.clone()
    }
}
