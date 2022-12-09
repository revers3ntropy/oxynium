use crate::ast::{Node, TypeCheckRes};
use crate::context::{Ctx, SymbolDec};
use crate::error::{Error, syntax_error};

#[derive(Debug)]
pub struct LocalVarNode {
    pub identifier: String,
    pub value: Box<dyn Node>,
    pub mutable: bool,
    pub local_var_idx: usize,
}

impl Node for LocalVarNode {
    fn asm(&mut self, ctx: Ctx) -> Result<String, Error> {
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
        ", self.value.asm(ctx)?, (self.local_var_idx+1) * 8))
    }

    fn type_check(&mut self, ctx: Ctx) -> Result<TypeCheckRes, Error> {
        let (type_, _) = self.value.type_check(ctx.clone())?;
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

#[derive(Debug)]
pub struct EmptyLocalVarNode {
    pub identifier: String,
    pub local_var_idx: usize,
    pub type_: Box<dyn Node>,
}

impl Node for EmptyLocalVarNode {
    fn asm(&mut self, ctx: Ctx) -> Result<String, Error> {
        if ctx.borrow_mut().stack_frame_peak().is_none() {
            return Err(syntax_error(format!(
                "Cannot declare local variable '{}' outside of function. Try using 'var' or 'const' instead.",
                self.identifier
            )));
        }
        Ok(format!(""))
    }

    fn type_check(&mut self, ctx: Ctx) -> Result<TypeCheckRes, Error> {
        self.local_var_idx = ctx.borrow_mut().get_declarations().len();

        let (type_, _) = self.type_.type_check(ctx.clone())?;

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

