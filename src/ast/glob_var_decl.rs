use crate::ast::Node;
use crate::ast::types::built_in::VOID;
use crate::ast::types::Type;
use crate::context::{Context, Symbol};
use crate::error::Error;

#[derive(Debug)]
pub struct GlobVarDecl<T> {
    pub identifier: String,
    pub value: T,
    pub type_: Box<Type>
}

impl Node for GlobVarDecl<i64> {
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error> {
        let data = format!("dq {}", self.value);
        ctx.declare(Symbol {
            name: self.identifier.clone(),
            data: Some(data),
            constant: false,
            type_: self.type_.clone()
        });
        Ok("".to_owned())
    }

    fn type_check(&mut self, _: &mut Context) -> Result<Box<Type>, Error> {
        Ok(Box::new(VOID))
    }
}