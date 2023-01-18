use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::{unknown_symbol, Error};
use crate::oxy_std::macros::asm::AsmMacro;
use crate::oxy_std::macros::Macro;
use crate::parse::token::Token;
use crate::position::Interval;
use crate::util::MutRc;
use std::rc::Rc;

#[derive(Debug)]
pub struct MacroCallNode {
    pub identifier: Token,
    pub args: Vec<MutRc<dyn Node>>,
    pub position: Interval,
}

impl MacroCallNode {
    pub fn get_macro(&self) -> Option<Rc<dyn Macro>> {
        match self
            .identifier
            .literal
            .as_ref()
            .unwrap()
            .as_str()
        {
            "asm" => Some(Rc::new(AsmMacro {
                position: self.position.clone(),
                args: self.args.clone(),
            })),
            _ => None,
        }
    }
}

impl Node for MacroCallNode {
    fn asm(
        &mut self,
        ctx: MutRc<Context>,
    ) -> Result<String, Error> {
        let macro_ = self.get_macro().unwrap();
        macro_
            .resolve(ctx.clone())?
            .borrow_mut()
            .asm(ctx.clone())
    }

    fn type_check(
        &self,
        ctx: MutRc<Context>,
    ) -> Result<TypeCheckRes, Error> {
        let macro_ = self.get_macro();
        if macro_.is_none() {
            return Err(unknown_symbol(format!(
                "macro `{}` does not exist",
                self.identifier.literal.as_ref().unwrap()
            ))
            .set_interval(self.position.clone()));
        }
        macro_
            .unwrap()
            .resolve(ctx.clone())?
            .borrow_mut()
            .type_check(ctx.clone())
    }

    fn pos(&self) -> Interval {
        self.position.clone()
    }
}
