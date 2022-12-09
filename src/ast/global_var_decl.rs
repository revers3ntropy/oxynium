use crate::ast::{Node, TypeCheckRes};
use crate::context::Ctx;
use crate::error::{Error, syntax_error};
use crate::symbols::{is_valid_identifier, SymbolDec, SymbolDef};

#[derive(Debug)]
pub struct GlobalConstNode<T> {
    pub identifier: String,
    pub value: T,
    pub is_const: bool
}

impl Node for GlobalConstNode<i64> {
    fn asm(&mut self, ctx: Ctx) -> Result<String, Error> {
        if ctx.borrow_mut().stack_frame_peak().is_some() {
            return Err(syntax_error(format!(
                "Cannot declare global {} '{}' inside function. Try using 'let' instead.",
                if self.is_const { "constant" } else { "variable" },
                self.identifier
            )));
        }
        ctx.borrow_mut().define(SymbolDef {
            name: self.identifier.clone(),
            data: Some(format!("dq {}", self.value)),
            text: None,
            is_local: false,
        }, false)?;
        Ok("".to_owned())
    }

    fn type_check(&mut self, ctx: Ctx) -> Result<TypeCheckRes, Error> {
        if !is_valid_identifier(&self.identifier) {
            return Err(syntax_error(format!(
                "Invalid global variable '{}'",
                self.identifier.clone()
            )));
        }
        let int = ctx.borrow_mut().get_dec_from_id("Int")?.type_.clone();
        ctx.borrow_mut().declare(SymbolDec {
            name: self.identifier.clone(),
            id: format!("qword [{}]", self.identifier.clone()),
            is_constant: self.is_const,
            is_type: false,
            require_init: true,
            is_defined: true,
            type_: int
        })?;
        Ok((ctx.borrow_mut().get_dec_from_id("Int")?.type_.clone(), None))
    }
}

impl Node for GlobalConstNode<String> {
    fn asm(&mut self, ctx: Ctx) -> Result<String, Error> {
        if ctx.borrow_mut().stack_frame_peak().is_some() {
            return Err(syntax_error(format!(
                "Cannot declare global {} '{}' inside function. Try using 'let' instead.",
                if self.is_const { "constant" } else { "variable" },
                self.identifier
            )));
        }
        ctx.borrow_mut().define(SymbolDef {
            name: self.identifier.clone(),
            // ,0 is the null terminator
            data: Some(format!("dq \"{}\", 0", self.value)),
            text: None,
            is_local: false,
        }, false)?;
        Ok("".to_owned())
    }

    fn type_check(&mut self, ctx: Ctx) -> Result<TypeCheckRes, Error> {
        if !is_valid_identifier(&self.identifier) {
            return Err(syntax_error(format!(
                "Invalid global variable '{}'",
                self.identifier.clone()
            )));
        }
        let str = ctx.borrow_mut().get_dec_from_id("Str")?.type_.clone();
        ctx.borrow_mut().declare(SymbolDec {
            name: self.identifier.clone(),
            id: self.identifier.clone(),
            is_constant: true,
            is_type: false,
            require_init: true,
            is_defined: true,
            type_: str
        })?;
        Ok((ctx.borrow_mut().get_dec_from_id("Str")?.type_.clone(), None))
    }
}