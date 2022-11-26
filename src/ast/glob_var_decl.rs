use crate::ast::Node;
use crate::context::{Context, Symbol};
use crate::error::Error;

#[derive(Debug)]
pub struct GlobVarDecl<T> {
    pub identifier: String,
    pub value: T
}

impl Node for GlobVarDecl<i64> {
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error> {
        let data = format!("dq {}", self.value);
        ctx.declare(Symbol {
            name: self.identifier.clone(),
            data: Some(data),
            constant: false
        });
        Ok("".to_owned())
    }
}