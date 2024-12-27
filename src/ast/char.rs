use crate::ast::{AstNode, TypeCheckRes};
use crate::context::Context;
use crate::error::Error;
use crate::parse::token::Token;
use crate::position::Interval;
use crate::util::MutRc;

#[derive(Debug, Clone)]
pub struct CharNode {
    pub value: Token,
}

impl CharNode {
    fn val(&mut self) -> String {
        self.value.literal.as_ref().unwrap().clone()
    }
}

impl AstNode for CharNode {
    fn type_check(&self, ctx: MutRc<dyn Context>) -> Result<TypeCheckRes, Error> {
        Ok(TypeCheckRes::from_type_in_ctx(&ctx, "Char", 0, true))
    }

    fn asm(&mut self, _ctx: MutRc<dyn Context>) -> Result<String, Error> {
        let str = self.val().clone();

        let mut asm = String::new();

        let mut bytes = vec![];
        for byte in str.chars().next().unwrap().to_string().bytes() {
            bytes.push(format!("{:x}", byte));
        }
        // pad to 8 elements
        while bytes.len() < 8 {
            bytes.push("00".to_string());
        }
        bytes.reverse();
        asm.push_str(&bytes.join(""));

        Ok(format!(
            "
                mov rax, 0x{asm}
                push rax
            "
        ))
    }

    fn pos(&self) -> Interval {
        self.value.interval()
    }
}
