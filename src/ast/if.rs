use crate::ast::{Node, TypeCheckRes};
use crate::context::Ctx;
use crate::error::{Error, type_error};

#[derive(Debug)]
pub struct IfNode {
    pub comparison: Box<dyn Node>,
    pub body: Box<dyn Node>,
    pub else_body: Option<Box<dyn Node>>
}

impl Node for IfNode {
    fn asm(&mut self, ctx: Ctx) -> Result<String, Error> {
        let body = self.body.asm(ctx.clone())?;
        let comp = self.comparison.asm(ctx.clone())?;
        let after_lbl = ctx.borrow_mut().get_anon_label();

        if self.else_body.is_some() {
            let else_body = self.else_body.take().unwrap().asm(ctx.clone())?;
            let else_lbl = ctx.borrow_mut().get_anon_label();

            Ok(format!("
                {comp}
                pop rax
                cmp rax, 0     ; if evaluates to false, don't do body
                je {else_lbl}
                {body}
                jmp {after_lbl}
                {else_lbl}:
                {else_body}
                {after_lbl}:
            "))
        } else {
            Ok(format!("
                {comp}
                pop rax
                cmp rax, 0     ; if evaluates to false, don't do body
                je {after_lbl}
                {body}
                {after_lbl}:
            "))
        }
    }

    fn type_check(&mut self, ctx: Ctx) -> Result<TypeCheckRes, Error> {
        let (_, mut body_ret_type) = self.body.type_check(ctx.clone())?;

        let (comp_type, _) = self.comparison.type_check(ctx.clone())?;
        if !ctx.borrow_mut().get_dec_from_id("Bool")?.type_.contains(comp_type) {
            return Err(type_error("if condition must be a bool".to_string()))
        }

        if self.else_body.is_some() {
            let mut else_body = self.else_body.take().unwrap();
            let (_, else_ret_type) = else_body.type_check(ctx.clone())?;
            self.else_body = Some(else_body);

            if let Some(body_ret) = body_ret_type.take() {
                if let Some(else_ret_type) = else_ret_type {
                    if !body_ret.contains(else_ret_type.clone()) {
                        return Err(type_error(format!(
                            "if statement branches cannot return different types: {} and {}",
                            body_ret.str(), else_ret_type.str()
                        )));
                    }
                }
                body_ret_type = Some(body_ret);
            } else if else_ret_type.is_some() {
                body_ret_type = else_ret_type;
            }
        }
        Ok((ctx.borrow_mut().get_dec_from_id("Void")?.type_.clone(), body_ret_type))
    }
}