use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::{syntax_error, type_error, Error};
use crate::get_type;
use crate::parse::token::Token;
use crate::position::Interval;
use crate::symbols::{can_declare_with_identifier, SymbolDec, SymbolDef};
use crate::util::MutRc;

#[derive(Debug)]
pub struct LocalVarNode {
    pub identifier: Token,
    pub value: MutRc<dyn Node>,
    pub mutable: bool,
    pub stack_offset: usize,
    pub type_annotation: Option<MutRc<dyn Node>>,
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
            text: None,
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
        if !can_declare_with_identifier(&self.id()) {
            return Err(syntax_error(format!(
                "Invalid local variable '{}'",
                self.id()
            ))
            .set_interval(self.identifier.interval()));
        }
        let (mut value_type, _) =
            self.value.borrow_mut().type_check(ctx.clone())?;
        self.stack_offset = ctx.borrow_mut().get_new_local_var_offset();

        if self.type_annotation.is_some() {
            let type_annotation = self.type_annotation.as_ref().unwrap();
            let (type_annotation_type, _) =
                type_annotation.borrow_mut().type_check(ctx.clone())?;
            if !type_annotation_type.borrow().contains(value_type.clone()) {
                return Err(type_error(format!(
                    "Cannot assign value of type '{}' to variable '{}' of type '{}'",
                    value_type.borrow().str(),
                    self.id(),
                    type_annotation_type.borrow().str()
                ))
                .set_interval(self.pos()));
            }
            value_type = type_annotation_type;
        }

        ctx.borrow_mut().declare(SymbolDec {
            name: self.id(),
            id: format!("qword [rbp - {}]", self.stack_offset),
            is_constant: !self.mutable,
            is_type: false,
            require_init: true,
            is_defined: true,
            is_param: false,
            type_: value_type.clone(),
            position: self.pos(),
        })?;
        Ok((get_type!(ctx, "Void"), None))
    }

    fn pos(&mut self) -> Interval {
        (
            self.identifier.start.clone(),
            self.value.borrow_mut().pos().1,
        )
    }
}
