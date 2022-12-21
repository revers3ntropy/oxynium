use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::{syntax_error, Error};
use crate::position::Interval;
use crate::symbols::{is_valid_identifier, SymbolDec};
use crate::util::MutRc;

#[derive(Debug)]
pub struct EmptyLocalVarNode {
    pub identifier: String,
    pub type_: MutRc<dyn Node>,
    pub position: Interval,
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

    fn type_check(&self, ctx: MutRc<Context>) -> Result<TypeCheckRes, Error> {
        if !is_valid_identifier(&self.identifier) {
            return Err(syntax_error(format!(
                "Invalid local variable '{}'",
                self.identifier.clone()
            ))
            .set_interval(self.position.clone()));
        }

        let TypeCheckRes {
            t: type_, unknowns, ..
        } = self.type_.borrow_mut().type_check(ctx.clone())?;

        let stack_offset = ctx.borrow_mut().get_new_local_var_offset();
        ctx.borrow_mut().declare(
            SymbolDec {
                name: self.identifier.clone(),
                id: format!("qword [rbp - {stack_offset}]"),
                is_constant: false,
                is_type: false,
                require_init: true,
                is_defined: false,
                is_param: false,
                type_: type_.clone(),
                position: self.pos(),
            },
            self.pos(),
        )?;
        Ok(TypeCheckRes::from(type_.clone(), unknowns))
    }

    fn pos(&self) -> Interval {
        self.position.clone()
    }
}
