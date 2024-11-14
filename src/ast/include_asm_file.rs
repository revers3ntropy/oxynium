use crate::ast::symbol_access::SymbolAccess;
use crate::ast::{AstNode, TypeCheckRes};
use crate::context::Context;
use crate::error::Error;
use crate::parse::token::Token;
use crate::position::Interval;
use crate::util::MutRc;

#[derive(Debug)]
pub struct IncludeAsmFileNode {
    pub file_path: Token,
}

impl AstNode for IncludeAsmFileNode {
    fn asm(&mut self, ctx: MutRc<dyn Context>) -> Result<String, Error> {
        ctx.borrow_mut()
            .include_asm(self.file_path.literal.clone().unwrap());
        Ok("".to_string())
    }

    fn pos(&self) -> Interval {
        self.file_path.interval()
    }
}
