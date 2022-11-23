use crate::ast::Node;
use crate::context::Context;
use crate::error::Error;

#[derive(Debug)]
pub struct ConstDecl <T> {
    identifier: String,
    value: T
}

impl <T> ConstDecl <T> {
    pub fn new(identifier: String, value: T) -> ConstDecl<T> {
        ConstDecl {
            identifier,
            value
        }
    }
}

impl Node for ConstDecl <i64> {
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error> {
        let data = format!("dq {}", self.value);
        ctx.declare_symbol(self.identifier.clone(), data);
        Ok("".to_owned())
    }
}

impl Node for ConstDecl <String> {
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error> {
        ctx.reserve_symbol(self.identifier.clone());
        // ', 0' is a null terminator
        let data = format!("dq \"{}\", 0", self.value);
        ctx.declare_symbol(self.identifier.clone(), data);
        Ok("".to_owned())
    }
}