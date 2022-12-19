use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::{syntax_error, Error};
use crate::position::Interval;
use crate::symbols::{can_declare_with_identifier, SymbolDec, SymbolDef};
use crate::util::MutRc;

#[derive(Debug)]
pub struct GlobalConstNode<T> {
    pub identifier: String,
    pub value: T,
    pub position: Interval,
}

impl Node for GlobalConstNode<i64> {
    fn asm(&mut self, ctx: MutRc<Context>) -> Result<String, Error> {
        if ctx.borrow_mut().stack_frame_peak().is_some() {
            return Err(syntax_error(format!(
                "Cannot declare global constant '{}' inside function. Try using 'let' instead.",
                self.identifier
            )));
        }
        ctx.borrow_mut().define(SymbolDef {
            name: self.identifier.clone(),
            data: Some(format!("dq {}", self.value)),
            text: None,
        })?;
        Ok("".to_owned())
    }

    fn type_check(
        &mut self,
        ctx: MutRc<Context>,
    ) -> Result<TypeCheckRes, Error> {
        if !can_declare_with_identifier(&self.identifier) {
            return Err(syntax_error(format!(
                "Invalid global variable '{}'",
                self.identifier.clone()
            )));
        }
        let int = ctx.borrow_mut().get_dec_from_id("Int")?.type_.clone();
        ctx.borrow_mut().declare(SymbolDec {
            name: self.identifier.clone(),
            id: format!("qword [{}]", self.identifier.clone()),
            is_constant: true,
            is_type: false,
            require_init: true,
            is_defined: true,
            is_param: false,
            type_: int,
            position: self.pos()
        })?;
        Ok((ctx.borrow_mut().get_dec_from_id("Int")?.type_.clone(), None))
    }

    fn pos(&mut self) -> Interval {
        self.position.clone()
    }
}

impl Node for GlobalConstNode<String> {
    fn asm(&mut self, ctx: MutRc<Context>) -> Result<String, Error> {
        if ctx.borrow_mut().stack_frame_peak().is_some() {
            return Err(syntax_error(format!(
                "Cannot declare global constant '{}' inside function. Try using 'let' instead.",
                self.identifier
            )));
        }
        ctx.borrow_mut().define(SymbolDef {
            name: self.identifier.clone(),
            // ,0 is the null terminator
            data: Some(format!("dq \"{}\", 0", self.value)),
            text: None,
        })?;
        ctx.borrow_mut().define_anon(SymbolDef {
            name: self.identifier.clone(),
            // ,0 is the null terminator
            data: Some(format!("dq \"{}\", 0", self.value)),
            text: None,
        })?;
        Ok("".to_owned())
    }

    fn type_check(
        &mut self,
        ctx: MutRc<Context>,
    ) -> Result<TypeCheckRes, Error> {
        if !can_declare_with_identifier(&self.identifier) {
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
            is_param: false,
            type_: str,
            position: self.pos(),
        })?;
        Ok((ctx.borrow_mut().get_dec_from_id("Str")?.type_.clone(), None))
    }

    fn pos(&mut self) -> Interval {
        self.position.clone()
    }
}
