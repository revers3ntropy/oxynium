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
        let anon_id = ctx.borrow_mut().define_anon(SymbolDef {
            name: "".to_string(),
            // ,0 is the null terminator
            data: Some(format!("dq \"{}\", 0", self.val())),
            text: None
        })?;

        Ok(format!(
            "
            push {anon_id}
        "
        ))
    }

    fn type_check(
        &mut self,
        ctx: MutRc<Context>,
    ) -> Result<TypeCheckRes, Error> {
        Ok((ctx.borrow_mut().get_dec_from_id("Str")?.type_.clone(), None))
    }

    fn pos(&mut self) -> Interval {
        self.value.interval()
    }
}
