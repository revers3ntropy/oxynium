use crate::ast::Node;
use crate::ast::types::Type;
use crate::context::{Ctx, SymbolDef};
use crate::error::Error;

#[derive(Debug)]
pub struct IntNode {
    pub value: i64
}

impl Node for IntNode {
    fn asm(&mut self, ctx: Ctx) -> Result<String, Error> {
        let data = format!("dq {}", self.value);
        let reference = ctx.borrow_mut().get_anon_id();
        ctx.borrow_mut().define(SymbolDef {
            name: reference.clone(),
            data: Some(data),
            text: None,
            is_local: false
        }, true)?;
        Ok(format!("push {}", reference))
    }

    fn type_check(&mut self, ctx: Ctx) -> Result<Box<Type>, Error> {
        Ok(ctx.borrow_mut().get_dec_from_id("Int")?.type_.clone())
    }
}