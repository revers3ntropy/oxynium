use std::rc::Rc;
use crate::ast::{Node, TypeCheckRes};
use crate::context::{Ctx, SymbolDec, SymbolDef};
use crate::error::{Error};

#[derive(Debug)]
pub struct GlobalConstNode<T> {
    pub identifier: String,
    pub value: T,
    pub is_const: bool,
}

impl Node for GlobalConstNode<i64> {
    fn asm(&mut self, ctx: Ctx) -> Result<String, Error> {
        ctx.borrow_mut().define(SymbolDef {
            name: self.identifier.clone(),
            data: Some(format!("dq {}", self.value)),
            text: None,
            is_local: false,
        }, false)?;
        Ok("".to_owned())
    }

    fn type_check(&mut self, ctx: Ctx) -> Result<TypeCheckRes, Error> {
        let int = ctx.borrow_mut().get_dec_from_id("Int")?.type_.clone();
        ctx.borrow_mut().declare(SymbolDec {
            name: self.identifier.clone(),
            id: format!("qword [{}]", self.identifier.clone()),
            is_constant: self.is_const,
            is_type: false,
            type_: int
        })?;
        Ok((ctx.borrow_mut().get_dec_from_id("Int")?.type_.clone(), None))
    }
}

impl Node for GlobalConstNode<String> {
    fn asm(&mut self, ctx: Ctx) -> Result<String, Error> {
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
        let str = ctx.borrow_mut().get_dec_from_id("Str")?.type_.clone();
        ctx.borrow_mut().declare(SymbolDec {
            name: self.identifier.clone(),
            id: self.identifier.clone(),
            is_constant: true,
            is_type: false,
            type_: str
        })?;
        Ok((ctx.borrow_mut().get_dec_from_id("Str")?.type_.clone(), None))
    }
}

#[derive(Debug)]
pub struct EmptyGlobalConstNode {
    pub identifier: String,
    pub type_: Box<dyn Node>,
    pub is_const: bool,
}

impl Node for EmptyGlobalConstNode {
    fn type_check(&mut self, ctx: Ctx) -> Result<TypeCheckRes, Error> {
        let (type_, _) = self.type_.type_check(Rc::clone(&ctx))?;

        let id = if !type_.is_ptr {
            // deref if it shouldn't stay as a pointer
            format!("qword [{}]", self.identifier.clone())
        } else {
            self.identifier.clone()
        };

        ctx.borrow_mut().declare(SymbolDec {
            name: self.identifier.clone(),
            id,
            is_constant: self.is_const,
            is_type: false,
            type_
        })?;
        Ok((ctx.borrow_mut().get_dec_from_id("Void")?.type_.clone(), None))
    }
}