use crate::ast::symbol_access::SymbolAccess;
use crate::ast::{AstNode, TypeCheckRes};
use crate::context::Context;
use crate::error::Error;
use crate::parse::token::Token;
use crate::position::Interval;
use crate::util::MutRc;

#[derive(Debug)]
pub struct RawAsmNode {
    pub asm: Token,
    pub return_type: MutRc<SymbolAccess>,
}

impl AstNode for RawAsmNode {
    fn type_check(
        &self,
        ctx: MutRc<Context>,
    ) -> Result<TypeCheckRes, Error> {
        let mut res =
            self.return_type.borrow().type_check(ctx)?;
        if res.t.borrow().is_unknown() {
            return Ok(res);
        }
        res.t = res
            .t
            .clone()
            .borrow()
            .as_type_type()
            .unwrap()
            .instance_type;
        Ok(res)
    }

    fn asm(
        &mut self,
        _ctx: MutRc<Context>,
    ) -> Result<String, Error> {
        Ok(self.asm.clone().literal.unwrap())
    }

    fn pos(&self) -> Interval {
        self.asm.interval()
    }
}
