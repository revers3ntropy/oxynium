use crate::ast::Node;
use crate::ast::types::Type;
use crate::context::{Context, Symbol};
use crate::error::{Error, type_error_unstructured};

#[derive(Debug)]
pub struct GlobVarDecl<T> {
    pub identifier: String,
    pub value: T,
    pub type_: Box<Type>
}

impl Node for GlobVarDecl<i64> {
    fn asm(&mut self, _: &mut Context) -> Result<String, Error> {
        Ok("".to_owned())
    }

    fn type_check(&mut self, ctx: &mut Context) -> Result<Box<Type>, Error> {
        if !ctx.allow_overrides && ctx.has_with_id(self.identifier.clone().as_str()) {
            return Err(type_error_unstructured(format!("Symbol {} is already defined", self.identifier)))
        }
        let data = format!("dq {}", self.value);
        ctx.declare(Symbol {
            name: self.identifier.clone(),
            data: Some(data),
            text: None,
            constant: false,
            type_: self.type_.clone()
        });
        Ok(ctx.get_from_id("Void").type_.clone())
    }
}