use crate::ast::Node;
use crate::ast::types::built_in::{INT, STR, VOID};
use crate::ast::types::Type;
use crate::context::{Context, Symbol};
use crate::error::Error;

#[derive(Debug)]
pub struct ConstDeclNode<T> {
    pub identifier: String,
    pub value: T,
    pub is_const: bool,
}

impl Node for ConstDeclNode<i64> {
    fn asm(&mut self, _: &mut Context) -> Result<String, Error> {
        Ok("".to_owned())
    }

    fn type_check(&mut self, ctx: &mut Context) -> Result<Box<Type>, Error> {
        ctx.declare_glob_var(Symbol {
            name: self.identifier.clone(),
            data: Some(format!("dq {}", self.value)),
            constant: self.is_const,
            type_: Box::new(INT)
        });
        Ok(Box::new(INT))
    }
}

impl Node for ConstDeclNode<String> {
    fn asm(&mut self, _: &mut Context) -> Result<String, Error> {
        Ok("".to_owned())
    }

    fn type_check(&mut self, ctx: &mut Context) -> Result<Box<Type>, Error> {
        ctx.declare_glob_var(Symbol {
            name: self.identifier.clone(),
            // ', 0' is a null terminator
            data: Some(format!("dq \"{}\", 0", self.value)),
            constant: true,
            type_: Box::new(STR)
        });
        Ok(Box::new(STR))
    }
}


#[derive(Debug)]
pub struct EmptyConstDeclNode {
    pub identifier: String,
    pub type_: Box<dyn Node>
}

impl Node for EmptyConstDeclNode {
    fn asm(&mut self, _: &mut Context) -> Result<String, Error> {
        Ok("".to_owned())
    }

    fn type_check(&mut self, ctx: &mut Context) -> Result<Box<Type>, Error> {
        let type_ = self.type_.type_check(ctx)?.clone();
        ctx.declare(Symbol {
            name: self.identifier.clone(),
            data: None,
            constant: true,
            type_
        });
        Ok(Box::new(VOID))
    }
}