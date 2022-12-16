use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::{syntax_error, Error};
use crate::parse::token::Token;
use crate::position::Interval;
use crate::symbols::{is_valid_identifier, SymbolDec, SymbolDef};
use crate::util::MutRc;

#[derive(Debug)]
pub struct LocalVarNode {
    pub identifier: Token,
    pub value: MutRc<dyn Node>,
    pub mutable: bool,
    pub stack_offset: usize,
}

impl LocalVarNode {
    fn id(&self) -> String {
        self.identifier.literal.as_ref().unwrap().clone()
    }
}

impl Node for LocalVarNode {
    fn asm(&mut self, ctx: MutRc<Context>) -> Result<String, Error> {
        if ctx.borrow_mut().stack_frame_peak().is_none() {
            return Err(syntax_error(format!(
                "Cannot declare local variable '{}' outside of function. Try using 'var' or 'const' instead.",
                self.identifier.literal.as_ref().unwrap()
            )));
        }

        // just so that space is made on stack
        ctx.borrow_mut().define(SymbolDef {
            name: self.identifier.str(),
            data: Some("".to_string()),
            text: None
        })?;

        Ok(format!(
            "
            {}
            pop rax
            mov qword [rbp - {}], rax
        ",
            self.value.borrow_mut().asm(ctx)?,
            self.stack_offset
        ))
    }

    fn type_check(
        &mut self,
        ctx: MutRc<Context>,
    ) -> Result<TypeCheckRes, Error> {
        if !is_valid_identifier(&self.id()) {
            return Err(syntax_error(format!(
                "Invalid local variable '{}'",
                self.id()
            ))
            .set_interval(self.identifier.interval()));
        }
        let (type_, _) = self.value.borrow_mut().type_check(ctx.clone())?;
        self.stack_offset = ctx.borrow_mut().get_new_local_var_offset();

        ctx.borrow_mut().declare(SymbolDec {
            name: self.id(),
            id: format!("qword [rbp - {}]", self.stack_offset),
            is_constant: !self.mutable,
            is_type: false,
            require_init: true,
            is_defined: true,
            is_param: false,
            type_: type_.clone(),
        })?;
        Ok((type_.clone(), None))
    }

    fn pos(&mut self) -> Interval {
        (
            self.identifier.start.clone(),
            self.value.borrow_mut().pos().1,
        )
    }
}
