use crate::ast::Node;
use crate::ast::types::Type;
use crate::context::{Context, SymbolDef};
use crate::error::Error;

#[derive(Debug)]
pub struct IntNode {
    pub value: i64
}

impl Node for IntNode {
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error> {
        let data = format!("dq {}", self.value);
        let reference = ctx.get_anon_id();
        ctx.define(SymbolDef {
            name: reference.clone(),
            data: Some(data),
            text: None,
            is_local: false
        }, true)?;
        Ok(format!("push {}", reference))
    }

    fn type_check(&mut self, ctx: &mut Context) -> Result<Box<Type>, Error> {
        Ok(ctx.get_dec_from_id("Int").type_.clone())
    }
}