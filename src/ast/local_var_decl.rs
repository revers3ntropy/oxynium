use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::util::MutRc;
use crate::error::{Error, syntax_error};
use crate::symbols::{is_valid_identifier, SymbolDec};

#[derive(Debug)]
pub struct LocalVarNode {
    pub identifier: String,
    pub value: MutRc<dyn Node>,
    pub mutable: bool,
    pub local_var_idx: usize,
}

impl Node for LocalVarNode {
    fn asm(&mut self, ctx: MutRc<Context>) -> Result<String, Error> {
        if ctx.borrow_mut().stack_frame_peak().is_none() {
            return Err(syntax_error(format!(
                "Cannot declare local variable '{}' outside of function. Try using 'var' or 'const' instead.",
                self.identifier
            )));
        }
        Ok(format!("
            {}
            pop rax
            mov qword [rbp - {}], rax
        ", self.value.borrow_mut().asm(ctx)?, (self.local_var_idx+1) * 8))
    }

    fn type_check(&mut self, ctx: MutRc<Context>) -> Result<TypeCheckRes, Error> {
        if !is_valid_identifier(&self.identifier) {
            return Err(syntax_error(format!(
                "Invalid local variable '{}'",
                self.identifier.clone()
            )));
        }
        let (type_, _) = self.value.borrow_mut().type_check(ctx.clone())?;
        self.local_var_idx = ctx.borrow_mut().get_declarations().len();

        ctx.borrow_mut().declare(SymbolDec {
            name: self.identifier.clone(),
            id: format!("qword [rbp - {}]", (self.local_var_idx+1) * 8),
            is_constant: !self.mutable,
            is_type: false,
            require_init: true,
            is_defined: true,
            type_: type_.clone()
        })?;
        Ok((type_.clone(), None))
    }
}
