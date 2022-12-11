use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::util::MutRc;
use crate::error::{Error, syntax_error};
use crate::symbols::{is_valid_identifier, SymbolDec};

#[derive(Debug)]
pub struct EmptyLocalVarNode {
    pub identifier: String,
    pub local_var_idx: usize,
    pub type_: MutRc<dyn Node>,
}

impl Node for EmptyLocalVarNode {
    fn asm(&mut self, ctx: MutRc<Context>) -> Result<String, Error> {
        if ctx.borrow_mut().stack_frame_peak().is_none() {
            return Err(syntax_error(format!(
                "Cannot declare local variable '{}' outside of function. Try using 'var' or 'const' instead.",
                self.identifier
            )));
        }
        Ok(format!(""))
    }

    fn type_check(&mut self, ctx: MutRc<Context>) -> Result<TypeCheckRes, Error> {
        if !is_valid_identifier(&self.identifier) {
            return Err(syntax_error(format!(
                "Invalid local variable '{}'",
                self.identifier.clone()
            )));
        }
        self.local_var_idx = ctx.borrow_mut().get_declarations().len();

        let (type_, _) = self.type_.borrow_mut().type_check(ctx.clone())?;

        ctx.borrow_mut().declare(SymbolDec {
            name: self.identifier.clone(),
            id: format!("qword [rbp - {}]", (self.local_var_idx+1) * 8),
            is_constant: false,
            is_type: false,
            require_init: true,
            is_defined: false,
            type_: type_.clone()
        })?;
        Ok((type_.clone(), None))
    }
}

