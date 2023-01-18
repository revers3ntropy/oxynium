use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::Error;
use crate::parse::token::Token;
use crate::position::Interval;
use crate::symbols::SymbolDef;
use crate::util::MutRc;

#[derive(Debug, Clone)]
pub struct StrNode {
    pub value: Token,
}

impl StrNode {
    fn val(&mut self) -> String {
        self.value.literal.as_ref().unwrap().clone()
    }
}

impl Node for StrNode {
    fn asm(
        &mut self,
        ctx: MutRc<Context>,
    ) -> Result<String, Error> {
        let str = self.val().clone();
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

        let anon_id = ctx.borrow_mut().define_anon(
            SymbolDef {
                name: "".to_string(),
                // ,0 is the null terminator
                data: Some(asm_str),
                text: None,
            },
            self.value.interval(),
        )?;

        Ok(format!(
            "
            push {anon_id}
        "
        ))
    }

    fn type_check(
        &self,
        ctx: MutRc<Context>,
    ) -> Result<TypeCheckRes, Error> {
        Ok(TypeCheckRes::from_ctx(&ctx, "Str", 0))
    }

    fn pos(&self) -> Interval {
        self.value.interval()
    }

    fn as_str_node(&self) -> Option<StrNode> {
        Some(self.clone())
    }
}
