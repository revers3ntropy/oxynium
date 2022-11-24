use crate::ast::Node;
use crate::context::Context;
use crate::error::Error;

#[derive(Debug)]
pub struct GlobVarDecl<T> {
    identifier: String,
    value: T
}

impl <T> GlobVarDecl<T> {
    pub fn new(identifier: String, value: T) -> GlobVarDecl<T> {
        GlobVarDecl {
            identifier,
            value
        }
    }
}

impl Node for GlobVarDecl<i64> {
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error> {
        let data = format!("dq {}", self.value);
        ctx.declare_symbol(self.identifier.clone(), data);
        Ok("".to_owned())
    }
}