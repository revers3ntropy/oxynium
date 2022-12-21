use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::Error;
use crate::parse::token::Token;
use crate::position::Interval;
use crate::symbols::SymbolDef;
use crate::util::MutRc;

#[derive(Debug)]
pub struct StrNode {
    pub value: Token,
}

impl StrNode {
    fn val(&mut self) -> String {
        self.value.literal.as_ref().unwrap().clone()
    }
}

impl Node for StrNode {
    fn asm(&mut self, ctx: MutRc<Context>) -> Result<String, Error> {
        let anon_id = ctx.borrow_mut().define_anon(
            SymbolDef {
                name: "".to_string(),
                // ,0 is the null terminator
                data: Some(format!("dq \"{}\", 0", self.val())),
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

    fn type_check(&self, ctx: MutRc<Context>) -> Result<TypeCheckRes, Error> {
        Ok(TypeCheckRes::from_ctx(&ctx, "Str", 0))
    }

    fn pos(&self) -> Interval {
        self.value.interval()
    }
}
