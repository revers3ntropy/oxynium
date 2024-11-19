use crate::ast::local_var_decl::LocalVarNode;
use crate::ast::{AstNode, TypeCheckRes};
use crate::context::Context;
use crate::error::Error;
use crate::parse::token::{Token, TokenType};
use crate::position::Interval;
use crate::util::{mut_rc, MutRc};

#[derive(Debug)]
pub struct ForRangeLoopNode {
    pub id_token: Token,
    pub start: MutRc<dyn AstNode>,
    pub end: MutRc<dyn AstNode>,
    pub step: MutRc<dyn AstNode>,
    pub statements: MutRc<dyn AstNode>,
    pub position: Interval,
    pub counter_decl_node: MutRc<dyn AstNode>,
    pub end_decl_node: MutRc<dyn AstNode>,
    pub step_decl_node: MutRc<dyn AstNode>,
}

impl ForRangeLoopNode {
    fn end_identifier(&self) -> Token {
        Token {
            token_type: TokenType::Identifier,
            literal: Some(format!(
                "_$__{}__end",
                self.id_token.literal.as_ref().unwrap()
            )),
            start: self.id_token.start.clone(),
            end: self.id_token.end.clone(),
        }
    }

    fn step_identifier(&self) -> Token {
        Token {
            token_type: TokenType::Identifier,
            literal: Some(format!(
                "_$__{}__step",
                self.id_token.literal.as_ref().unwrap()
            )),
            start: self.id_token.start.clone(),
            end: self.id_token.end.clone(),
        }
    }
}

impl AstNode for ForRangeLoopNode {
    fn setup(&mut self, ctx: MutRc<dyn Context>) -> Result<(), Error> {
        self.start.borrow_mut().setup(ctx.clone())?;
        self.end.borrow_mut().setup(ctx.clone())?;
        self.step.borrow_mut().setup(ctx.clone())?;

        self.counter_decl_node = mut_rc(LocalVarNode {
            identifier: self.id_token.clone(),
            value: self.start.clone(),
            mutable: false,
            type_annotation: None,
            start: self.id_token.start.clone(),
            allow_anon_identifier: false,
        });
        self.counter_decl_node.borrow_mut().setup(ctx.clone())?;

        self.end_decl_node = mut_rc(LocalVarNode {
            identifier: self.end_identifier(),
            value: self.end.clone(),
            mutable: false,
            type_annotation: None,
            start: self.id_token.start.clone(),
            allow_anon_identifier: true,
        });
        self.end_decl_node.borrow_mut().setup(ctx.clone())?;

        self.step_decl_node = mut_rc(LocalVarNode {
            identifier: self.step_identifier(),
            value: self.step.clone(),
            mutable: false,
            type_annotation: None,
            start: self.id_token.start.clone(),
            allow_anon_identifier: true,
        });
        self.step_decl_node.borrow_mut().setup(ctx.clone())?;

        self.statements.borrow_mut().setup(ctx.clone())
    }
    fn type_check(&self, ctx: MutRc<dyn Context>) -> Result<TypeCheckRes, Error> {
        let TypeCheckRes { mut unknowns, .. } = self
            .counter_decl_node
            .borrow_mut()
            .type_check(ctx.clone())?;

        let TypeCheckRes {
            unknowns: end_unknowns,
            ..
        } = self.end_decl_node.borrow_mut().type_check(ctx.clone())?;
        unknowns += end_unknowns;

        let TypeCheckRes {
            unknowns: step_unknowns,
            ..
        } = self.step_decl_node.borrow_mut().type_check(ctx.clone())?;
        unknowns += step_unknowns;

        let mut statements_tr = self.statements.borrow_mut().type_check(ctx.clone())?;
        statements_tr.unknowns += unknowns;
        Ok(statements_tr)
    }

    fn asm(&mut self, ctx: MutRc<dyn Context>) -> Result<String, Error> {
        let start_lbl = ctx.borrow_mut().get_anon_label();
        let end_lbl = ctx.borrow_mut().get_anon_label();

        let counter_dec_asm = self.counter_decl_node.borrow_mut().asm(ctx.clone())?;
        let end_dec_asm = self.end_decl_node.borrow_mut().asm(ctx.clone())?;
        let step_dec_asm = self.step_decl_node.borrow_mut().asm(ctx.clone())?;

        ctx.borrow_mut()
            .loop_labels_push(start_lbl.clone(), end_lbl.clone());

        // loop label exists on loop label stack just inside loop body
        let body = self.statements.borrow_mut().asm(ctx.clone())?;

        ctx.borrow_mut().loop_labels_pop();

        let counter_id = self.id_token.literal.as_ref().unwrap();

        let get_counter_asm = ctx.borrow().get_dec_from_id(&counter_id.clone()).id;
        let get_end_asm = ctx
            .borrow()
            .get_dec_from_id(&self.end_identifier().literal.unwrap())
            .id;
        let get_step_asm = ctx
            .borrow()
            .get_dec_from_id(&self.step_identifier().literal.unwrap())
            .id;

        Ok(format!(
            "
                {counter_dec_asm}
                {end_dec_asm}
                {step_dec_asm}
                {start_lbl}:
                    mov rax, {get_counter_asm}
                    cmp rax, {get_end_asm}
                    jge {end_lbl}
                    {body}
                    mov rax, {get_counter_asm}
                    add rax, {get_step_asm}
                    mov {get_counter_asm}, rax
                    jmp {start_lbl}
                {end_lbl}:
            "
        ))
    }

    fn pos(&self) -> Interval {
        self.position.clone()
    }
}
