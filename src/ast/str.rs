use crate::ast::{Node, TypeCheckRes};
use crate::context::Ctx;
use crate::error::Error;

#[derive(Debug)]
pub struct StrNode {
    pub value: String
}

impl Node for StrNode {
    fn asm(&mut self, _ctx: Ctx) -> Result<String, Error> {
        let mut result = format!("
            mov rdi, {}
            call malloc WRT ..plt
            push rax
        ",
            // + 1 for null terminator
           (self.value.len() + 1) * 8);

        let mut i = 0;
        for char in self.value.chars() {
            let char_res = format!("
                mov qword [rax+{}], '{}'
            ", i*8, char);
            result = format!("{}{}", result, char_res);
            i += 1;
        }
        format!("
            mov qword [rax+{}], 0
        ", i*8);

        Ok(result)
    }

    fn type_check(&mut self, ctx: Ctx) -> Result<TypeCheckRes, Error> {
        Ok((ctx.borrow_mut().get_dec_from_id("Str")?.type_.clone(), None))
    }
}