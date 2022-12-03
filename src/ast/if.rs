use crate::ast::Node;
use crate::ast::types::Type;
use crate::context::Context;
use crate::error::Error;

#[derive(Debug)]
pub struct IfNode {
    pub comparison: Box<dyn Node>,
    pub body: Box<dyn Node>,
    pub else_body: Option<Box<dyn Node>>
}

impl Node for IfNode {
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error> {
        let body = self.body.asm(ctx)?;
        let comp = self.comparison.asm(ctx)?;
        let after_lbl = ctx.get_anon_label();

        if self.else_body.is_some() {
            let else_body = self.else_body.take().unwrap().asm(ctx)?;
            let else_lbl = ctx.get_anon_label();

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

    fn type_check(&mut self, ctx: &mut Context) -> Result<Box<Type>, Error> {
        self.body.type_check(ctx)?;
        self.comparison.type_check(ctx)?;
        Ok(ctx.get_dec_from_id("Void").type_.clone())
    }
}