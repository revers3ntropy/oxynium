use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::Error;
use crate::parse::token::Token;
use crate::position::Interval;
use crate::util::MutRc;

#[derive(Debug)]
pub struct StrNode {
    pub value: Token
}

impl Node for StrNode {
    fn asm(&mut self, _ctx: MutRc<Context>) -> Result<String, Error> {
        let mut result = format!("
            mov rdi, {}
            call malloc WRT ..plt
        ",
            // + 1 for null terminator
           (self.value.literal.as_ref().unwrap().len() + 1) * 8);

        let mut i = 0;
        for char in self.value.literal.as_ref().unwrap().chars() {
            let char_res = format!("
                mov qword [rax+{}], '{}'
            ", i*8, char);
            result = format!("{}{}", result, char_res);
            i += 1;
        }
        result = format!("
            {result}
            mov qword [rax+{}], 0
            push rax
        ", i*8);

        Ok(result)
    }

    fn type_check(&mut self, ctx: MutRc<Context>) -> Result<TypeCheckRes, Error> {
        Ok((ctx.borrow_mut().get_dec_from_id("Str")?.type_.clone(), None))
    }

    fn pos(&mut self) -> Interval {
        self.value.interval()
    }
}