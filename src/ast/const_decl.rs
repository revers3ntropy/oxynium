use crate::ast::Node;
use crate::ast::types::Type;
use crate::context::{Context, Symbol};
use crate::error::{Error};

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
        if ctx.has_with_id(self.identifier.clone().as_str()) {
            //return Err(type_error_unstructured(format!("Symbol {} is already defined", self.identifier)))
        }
        ctx.declare_glob_var(Symbol {
            name: self.identifier.clone(),
            data: Some(format!("dq {}", self.value)),
            text: None,
            constant: self.is_const,
            type_: ctx.get_from_id("Int").type_.clone()
        });
        Ok(ctx.get_from_id("Int").type_.clone())
    }
}

impl Node for ConstDeclNode<String> {
    fn asm(&mut self, _: &mut Context) -> Result<String, Error> {
        Ok("".to_owned())
    }

    fn type_check(&mut self, ctx: &mut Context) -> Result<Box<Type>, Error> {
        if ctx.has_with_id(self.identifier.clone().as_str()) {
            //return Err(type_error_unstructured(format!("Symbol {} is already defined", self.identifier)))
        }
        ctx.declare_glob_var(Symbol {
            name: self.identifier.clone(),
            // ', 0' is a null terminator
            data: Some(format!("dq \"{}\", 0", self.value)),
            text: None,
            constant: true,
            type_: ctx.get_from_id("Str").type_.clone()
        });
        Ok(ctx.get_from_id("Str").type_.clone())
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
        if ctx.has_with_id(self.identifier.clone().as_str()) {
            //return Err(type_error_unstructured(format!("Symbol {} is already defined", self.identifier)))
        }
        let type_ = self.type_.type_check(ctx)?.clone();
        ctx.declare(Symbol {
            name: self.identifier.clone(),
            data: None,
            text: None,
            constant: true,
            type_
        });
        Ok(ctx.get_from_id("Void").type_.clone())
    }
}