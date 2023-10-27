use crate::ast::{AstNode, TypeCheckRes};
use crate::context::Context;
use crate::error::{syntax_error, Error};
use crate::get_type;
use crate::parse::token::Token;
use crate::position::Interval;
use crate::symbols::{can_declare_with_identifier, SymbolDec, SymbolDef};
use crate::types::unknown::UnknownType;
use crate::util::{new_mut_rc, MutRc};

#[derive(Debug)]
pub struct GlobalConstNode<T> {
    pub identifier: Token,
    pub value: T,
    pub position: Interval,
    pub is_exported: bool,
}

impl<T> GlobalConstNode<T> {
    fn asm_id(&self) -> String {
        format!("_$_gc_{}", self.identifier.clone().literal.unwrap())
    }
}

impl AstNode for GlobalConstNode<i64> {
    fn type_check(&self, ctx: MutRc<dyn Context>) -> Result<TypeCheckRes, Error> {
        if !can_declare_with_identifier(&self.identifier.clone().literal.unwrap()) {
            return Err(syntax_error(format!(
                "invalid global variable '{}'",
                self.identifier.clone().literal.unwrap()
            ))
            .set_interval(self.identifier.interval()));
        }
        let int = get_type!(ctx, "Int");
        ctx.borrow_mut().declare(
            SymbolDec {
                name: self.identifier.clone().literal.unwrap(),
                id: format!("qword [rel {}]", self.asm_id()),
                is_constant: true,
                is_type: false,
                is_func: false,
                require_init: true,
                is_defined: true,
                is_param: false,
                type_: int,
                position: self.pos(),
            },
            self.identifier.interval(),
        )?;
        Ok(TypeCheckRes::from_ctx(&ctx, "Int", 0, true))
    }

    fn asm(&mut self, ctx: MutRc<dyn Context>) -> Result<String, Error> {
        if ctx.borrow_mut().stack_frame_peak().is_some() {
            return Err(syntax_error(format!(
                "cannot declare global constant '{}' inside function",
                self.identifier.clone().literal.unwrap()
            )).hint("try using `let` instead".to_string())
            .set_interval((self.pos().0, self.identifier.end.clone())));
        }
        ctx.borrow_mut().define(
            SymbolDef {
                name: self.asm_id(),
                data: Some(format!("dq {}", self.value)),
                text: None,
            },
            self.pos(),
        )?;
        Ok("".to_owned())
    }

    fn pos(&self) -> Interval {
        self.position.clone()
    }
}

impl AstNode for GlobalConstNode<String> {
    fn asm(&mut self, ctx: MutRc<dyn Context>) -> Result<String, Error> {
        if ctx.borrow_mut().stack_frame_peak().is_some() {
            return Err(syntax_error(format!(
                "cannot declare global constant '{}' inside function. Try using 'let' instead.",
                self.identifier.clone().literal.unwrap()
            ))
            .set_interval((self.pos().0, self.identifier.end.clone())));
        }

        let str = self.value.clone();
        let symbols = str.chars();

        let mut asm = String::new();
        for symbol in symbols.clone() {
            let mut bytes = vec![];
            for byte in symbol.to_string().bytes() {
                bytes.push(format!("0x{:x}", byte));
            }
            // pad to 8 elements
            while bytes.len() < 8 {
                bytes.push("0x0".to_string());
            }
            asm.push_str(&bytes.join(","));
            asm.push_str(",");
        }
        asm.pop();

        let asm_str = if symbols.into_iter().count() < 1 {
            format!("dq 0x0")
        } else {
            format!("db {} \ndq 0x0", asm)
        };

        ctx.borrow_mut().define(
            SymbolDef {
                name: self.asm_id(),
                data: Some(asm_str),
                text: None,
            },
            self.identifier.interval(),
        )?;
        Ok("".to_owned())
    }

    fn type_check(&self, ctx: MutRc<dyn Context>) -> Result<TypeCheckRes, Error> {
        if !can_declare_with_identifier(&self.identifier.clone().literal.unwrap()) {
            return Err(syntax_error(format!(
                "invalid global variable '{}'",
                self.identifier.clone().literal.unwrap()
            )));
        }
        let str = get_type!(ctx, "Str");
        ctx.borrow_mut().declare(
            SymbolDec {
                name: self.identifier.clone().literal.unwrap(),
                //id: self.asm_id(),
                id: format!("rel {}", self.asm_id()),
                is_constant: true,
                is_type: false,
                is_func: false,
                require_init: true,
                is_defined: true,
                is_param: false,
                type_: str,
                position: self.pos(),
            },
            self.identifier.interval(),
        )?;
        Ok(TypeCheckRes::from_ctx(&ctx, "Str", 0, true))
    }

    fn pos(&self) -> Interval {
        self.position.clone()
    }
}
