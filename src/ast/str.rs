use crate::ast::{AstNode, TypeCheckRes};
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

impl AstNode for StrNode {
    fn type_check(&self, ctx: MutRc<dyn Context>) -> Result<TypeCheckRes, Error> {
        Ok(TypeCheckRes::from_ctx(&ctx, "Str", 0, true))
    }

    fn asm(&mut self, ctx: MutRc<dyn Context>) -> Result<String, Error> {
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

        // null terminator
        let asm_str = if symbols.into_iter().count() < 1 {
            format!("default rel dq 0x0")
        } else {
            format!("default rel db {} \ndq 0x0", asm)
        };

        let anon_label = ctx.borrow_mut().get_anon_label();
        let symbol_name = if let Some(scope) = ctx.borrow().stack_frame_peak() {
            format!("{}{}", scope.name, anon_label)
        } else {
            // if not currently inside a function (ie, top level statement)
            format!("_main{}", anon_label)
        };

        ctx.borrow_mut().define_global(
            SymbolDef {
                name: symbol_name.clone(),
                data: Some(asm_str),
                text: None,
            },
            self.value.interval(),
        )?;

        Ok(format!(
            "
                lea rax, [{symbol_name}]
                push rax
            "
        ))
    }

    fn pos(&self) -> Interval {
        self.value.interval()
    }

    fn as_str_node(&self) -> Option<StrNode> {
        Some(self.clone())
    }
}
