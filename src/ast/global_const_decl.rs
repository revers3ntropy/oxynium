use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::{syntax_error, Error};
use crate::get_type;
use crate::parse::token::Token;
use crate::position::Interval;
use crate::symbols::{can_declare_with_identifier, SymbolDec, SymbolDef};
use crate::util::MutRc;

#[derive(Debug)]
pub struct GlobalConstNode<T> {
    pub identifier: Token,
    pub value: T,
    pub position: Interval,
}

impl Node for GlobalConstNode<i64> {
    fn asm(&mut self, ctx: MutRc<Context>) -> Result<String, Error> {
        if ctx.borrow_mut().stack_frame_peak().is_some() {
            return Err(syntax_error(format!(
                "Cannot declare global constant '{}' inside function. Try using 'let' instead.",
                self.identifier.clone().literal.unwrap()
            )).set_interval((self.pos().0, self.identifier.end.clone())));
        }
        ctx.borrow_mut().define(
            SymbolDef {
                name: self.identifier.clone().literal.unwrap(),
                data: Some(format!("dq {}", self.value)),
                text: None,
            },
            self.pos(),
        )?;
        Ok("".to_owned())
    }

    fn type_check(&self, ctx: MutRc<Context>) -> Result<TypeCheckRes, Error> {
        if !can_declare_with_identifier(
            &self.identifier.clone().literal.unwrap(),
        ) {
            return Err(syntax_error(format!(
                "Invalid global variable '{}'",
                self.identifier.clone().literal.unwrap()
            ))
            .set_interval(self.identifier.interval()));
        }
        let int = get_type!(ctx, "Int");
        ctx.borrow_mut().declare(
            SymbolDec {
                name: self.identifier.clone().literal.unwrap(),
                id: format!(
                    "qword [{}]",
                    self.identifier.clone().literal.unwrap()
                ),
                is_constant: true,
                is_type: false,
                require_init: true,
                is_defined: true,
                is_param: false,
                type_: int,
                position: self.pos(),
            },
            self.identifier.interval(),
        )?;
        Ok(TypeCheckRes::from_ctx(&ctx, "Int", 0))
    }

    fn pos(&self) -> Interval {
        self.position.clone()
    }
}

impl Node for GlobalConstNode<String> {
    fn asm(&mut self, ctx: MutRc<Context>) -> Result<String, Error> {
        if ctx.borrow_mut().stack_frame_peak().is_some() {
            return Err(syntax_error(format!(
                "Cannot declare global constant '{}' inside function. Try using 'let' instead.",
                self.identifier.clone().literal.unwrap()
            )).set_interval((self.pos().0, self.identifier.end.clone())));
        }
        ctx.borrow_mut().define(
            SymbolDef {
                name: self.identifier.clone().literal.unwrap(),
                // ,0 is the null terminator
                data: Some(format!("dq \"{}\", 0", self.value)),
                text: None,
            },
            self.identifier.interval(),
        )?;
        ctx.borrow_mut().define_anon(
            SymbolDef {
                name: self.identifier.clone().literal.unwrap(),
                // ,0 is the null terminator
                data: Some(format!("dq \"{}\", 0", self.value)),
                text: None,
            },
            self.pos(),
        )?;
        Ok("".to_owned())
    }

    fn type_check(&self, ctx: MutRc<Context>) -> Result<TypeCheckRes, Error> {
        if !can_declare_with_identifier(
            &self.identifier.clone().literal.unwrap(),
        ) {
            return Err(syntax_error(format!(
                "Invalid global variable '{}'",
                self.identifier.clone().literal.unwrap()
            )));
        }
        let str = get_type!(ctx, "Str");
        ctx.borrow_mut().declare(
            SymbolDec {
                name: self.identifier.clone().literal.unwrap(),
                id: self.identifier.clone().literal.unwrap(),
                is_constant: true,
                is_type: false,
                require_init: true,
                is_defined: true,
                is_param: false,
                type_: str,
                position: self.pos(),
            },
            self.identifier.interval(),
        )?;
        Ok(TypeCheckRes::from_ctx(&ctx, "Str", 0))
    }

    fn pos(&self) -> Interval {
        self.position.clone()
    }
}
