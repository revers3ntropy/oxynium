use crate::ast::Node;
use crate::context::{Context, Symbol};
use crate::error::Error;

#[derive(Debug)]
pub struct ConstDeclNode<T> {
    pub identifier: String,
    pub value: T
}

impl Node for ConstDeclNode<i64> {
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error> {
        ctx.declare_glob_var(Symbol {
            name: self.identifier.clone(),
            data: Some(format!("dq {}", self.value)),
            constant: true
        });
        Ok("".to_owned())
    }
}

impl Node for ConstDeclNode<String> {
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error> {
        ctx.declare_glob_var(Symbol {
            name: self.identifier.clone(),
            // ', 0' is a null terminator
            data: Some(format!("dq \"{}\", 0", self.value)),
            constant: true
        });
        Ok("".to_owned())
    }
}


#[derive(Debug)]
pub struct EmptyConstDeclNode {
    pub identifier: String
}

impl Node for EmptyConstDeclNode {
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error> {
        ctx.declare(Symbol {
            name: self.identifier.clone(),
            data: None,
            constant: true
        });
        Ok("".to_owned())
    }
}