use crate::ast::Node;
use crate::ast::types::Type;
use crate::context::{Context, SymbolDec, SymbolDef};
use crate::error::{Error, type_error_unstructured};

#[derive(Debug)]
pub struct GlobalConstNode<T> {
    pub identifier: String,
    pub value: T,
    pub is_const: bool,
}

impl Node for GlobalConstNode<i64> {
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error> {
        ctx.define(SymbolDef {
            name: self.identifier.clone(),
            data: Some(format!("dq {}", self.value)),
            text: None,
            is_local: false,
        }, false)?;
        Ok("".to_owned())
    }

    fn type_check(&mut self, ctx: &mut Context) -> Result<Box<Type>, Error> {
        if !ctx.allow_overrides && ctx.has_dec_with_id(self.identifier.clone().as_str()) {
            return Err(type_error_unstructured(format!("Symbol {} is already defined", self.identifier)))
        }
        ctx.declare(SymbolDec {
            name: self.identifier.clone(),
            is_constant: self.is_const,
            is_type: false,
            type_: ctx.get_dec_from_id("Int").type_.clone()
        })?;
        Ok(ctx.get_dec_from_id("Int").type_.clone())
    }
}

impl Node for GlobalConstNode<String> {
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error> {
        ctx.define(SymbolDef {
            name: self.identifier.clone(),
            // ,0 is the null terminator
            data: Some(format!("dq \"{}\", 0", self.value)),
            text: None,
            is_local: false,
        }, false)?;
        Ok("".to_owned())
    }

    fn type_check(&mut self, ctx: &mut Context) -> Result<Box<Type>, Error> {
        ctx.declare(SymbolDec {
            name: self.identifier.clone(),
            is_constant: true,
            is_type: false,
            type_: ctx.get_dec_from_id("Str").type_.clone()
        })?;
        Ok(ctx.get_dec_from_id("Str").type_.clone())
    }
}

#[derive(Debug)]
pub struct EmptyGlobalConstNode {
    pub identifier: String,
    pub type_: Box<dyn Node>,
    pub is_const: bool,
}

impl Node for EmptyGlobalConstNode {
    fn asm(&mut self, _: &mut Context) -> Result<String, Error> {
        Ok("".to_owned())
    }

    fn type_check(&mut self, ctx: &mut Context) -> Result<Box<Type>, Error> {
        let type_ = self.type_.type_check(ctx)?;
        ctx.declare(SymbolDec {
            name: self.identifier.clone(),
            is_constant: self.is_const,
            is_type: false,
            type_
        })?;
        Ok(ctx.get_dec_from_id("Void").type_.clone())
    }
}