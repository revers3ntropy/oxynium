use crate::ast::{AstNode, TypeCheckRes};
use crate::context::Context;
use crate::error::{syntax_error, type_error, Error};
use crate::parse::token::Token;
use crate::position::{Interval, Position};
use crate::symbols::{
    can_declare_with_identifier, SymbolDec, SymbolDef,
};
use crate::util::MutRc;

#[derive(Debug)]
pub struct LocalVarNode {
    pub identifier: Token,
    pub value: MutRc<dyn AstNode>,
    pub mutable: bool,
    pub type_annotation: Option<MutRc<dyn AstNode>>,
    pub start: Position,
}

impl LocalVarNode {
    fn id(&self) -> String {
        self.identifier.literal.as_ref().unwrap().clone()
    }
}

impl AstNode for LocalVarNode {
    fn setup(
        &mut self,
        ctx: MutRc<Context>,
    ) -> Result<(), Error> {
        if !can_declare_with_identifier(&self.id()) {
            return Err(syntax_error(format!(
                "Invalid local variable '{}'",
                self.id()
            ))
            .set_interval(self.identifier.interval()));
        }
        if self.type_annotation.is_some() {
            self.type_annotation
                .as_ref()
                .unwrap()
                .borrow_mut()
                .setup(ctx.clone())?;
        }
        self.value.borrow_mut().setup(ctx.clone())
    }
    fn type_check(
        &self,
        ctx: MutRc<Context>,
    ) -> Result<TypeCheckRes, Error> {
        let TypeCheckRes {
            t: mut value_type,
            mut unknowns,
            ..
        } = self.value.borrow().type_check(ctx.clone())?;

        if self.type_annotation.is_some() {
            let type_annotation =
                self.type_annotation.as_ref().unwrap();
            let TypeCheckRes {
                t: type_annotation_type,
                unknowns: type_annotation_unknowns,
                ..
            } = type_annotation
                .borrow()
                .type_check(ctx.clone())?;
            unknowns += type_annotation_unknowns;

            if !type_annotation_type
                .borrow()
                .contains(value_type.clone())
            {
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

        if ctx.borrow().is_frozen() {
            // update with latest type
            ctx.borrow_mut().update_dec_type(
                &self.id(),
                value_type.clone(),
                self.pos(),
            )?;
        } else {
            let offset =
                ctx.borrow_mut().get_new_local_var_offset();
            ctx.borrow_mut().declare(
                SymbolDec {
                    name: self.id(),
                    id: format!("qword [rbp - {offset}]"),
                    is_constant: !self.mutable,
                    is_type: false,
                    require_init: true,
                    is_defined: false,
                    is_param: false,
                    type_: value_type.clone(),
                    position: self.pos(),
                },
                self.identifier.interval(),
            )?;
        }

        Ok(TypeCheckRes::from_ctx(
            &ctx, "Void", unknowns, true,
        ))
    }

    fn asm(
        &mut self,
        ctx: MutRc<Context>,
    ) -> Result<String, Error> {
        if ctx.borrow_mut().stack_frame_peak().is_none() {
            return Err(syntax_error(format!(
                "Cannot declare local variable '{}' outside of function. Try using 'const' instead.",
                self.identifier.literal.as_ref().unwrap()
            )).set_interval((self.pos().0, self.identifier.end.clone())));
        }

        // just so that space is made on stack
        ctx.borrow_mut().define(
            SymbolDef {
                name: self.identifier.str(),
                data: Some("".to_string()),
                text: None,
            },
            self.pos(),
        )?;

        let id =
            ctx.borrow_mut().get_dec_from_id(&self.id()).id;

        let value_asm =
            self.value.borrow_mut().asm(ctx.clone())?;
        ctx.borrow_mut()
            .set_dec_as_defined(&self.id(), self.pos())?;

        Ok(format!(
            "
            {value_asm}
            pop rax
            mov {id}, rax
        "
        ))
    }

    fn pos(&self) -> Interval {
        (
            self.start.clone(),
            self.value.borrow_mut().pos().1,
        )
    }
}
