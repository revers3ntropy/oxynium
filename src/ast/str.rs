use crate::ast::Node;
use crate::ast::types::Type;
use crate::context::{Context, SymbolDef};
use crate::error::Error;

#[derive(Debug)]
pub struct StrNode {
    pub value: String
}

impl Node for StrNode {
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error> {
        let data = format!("dq \"{}\", 0", self.value);
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
        Ok(ctx.get_dec_from_id("Str").type_.clone())
    }
}