use std::rc::Rc;
use crate::ast::{Node, TypeCheckRes};
use crate::context::{Ctx};
use crate::error::{Error, type_error};

#[derive(Debug)]
pub struct IfNode {
    pub comparison: Box<dyn Node>,
    pub body: Box<dyn Node>,
    pub else_body: Option<Box<dyn Node>>
}

impl Node for IfNode {
    fn asm(&mut self, ctx: Ctx) -> Result<String, Error> {
        let body = self.body.asm(Rc::clone(&ctx))?;
        let comp = self.comparison.asm(Rc::clone(&ctx))?;
        let after_lbl = ctx.borrow_mut().get_anon_label();

        if self.else_body.is_some() {
            let else_body = self.else_body.take().unwrap().asm(Rc::clone(&ctx))?;
            let else_lbl = ctx.borrow_mut().get_anon_label();

            Ok(format!("
                {comp}
                pop rax
                mov rax, [rax]
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
                mov rax, [rax]
                cmp rax, 0     ; if evaluates to false, don't do body
                je {after_lbl}
                {body}
                {after_lbl}:
            "))
        }
    }

    fn type_check(&mut self, ctx: Ctx) -> Result<TypeCheckRes, Error> {
        let (_, mut body_ret_type) = self.body.type_check(Rc::clone(&ctx))?;

        self.comparison.type_check(Rc::clone(&ctx))?;

        if self.else_body.is_some() {
            let mut else_body = self.else_body.take().unwrap();
            let (_, else_ret_type) = else_body.type_check(Rc::clone(&ctx))?;
            self.else_body = Some(else_body);

            if let Some(body_ret) = body_ret_type.take() {
                if else_ret_type.is_some() && !body_ret.contains(&else_ret_type.as_ref().unwrap()) {
                    return Err(type_error(format!(
                        "if statement branches cannot return different types: {} and {}",
                        body_ret, else_ret_type.unwrap()
                    )));
                }
                body_ret_type = Some(body_ret);
            } else if else_ret_type.is_some() {
                body_ret_type = else_ret_type;
            }
        }
        Ok((ctx.borrow_mut().get_dec_from_id("Void")?.type_.clone(), body_ret_type))
    }
}