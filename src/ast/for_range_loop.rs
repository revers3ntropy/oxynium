use crate::ast::int::IntNode;
use crate::ast::local_var_decl::LocalVarNode;
use crate::ast::{AstNode, TypeCheckRes};
use crate::context::{Context, LoopLabels};
use crate::error::Error;
use crate::parse::token::{Token, TokenType};
use crate::position::Interval;
use crate::util::{mut_rc, MutRc};

#[derive(Debug)]
pub struct ForRangeLoopNode {
    pub value_tok: Token,
    pub counter_tok: Option<Token>,
    pub start: MutRc<dyn AstNode>,
    pub end: MutRc<dyn AstNode>,
    pub step: MutRc<dyn AstNode>,
    pub statements: MutRc<dyn AstNode>,
    pub position: Interval,
    pub value_decl_node: MutRc<dyn AstNode>,
    pub counter_decl_node: Option<MutRc<dyn AstNode>>,
    pub end_decl_node: MutRc<dyn AstNode>,
    pub step_decl_node: MutRc<dyn AstNode>,
}

impl ForRangeLoopNode {
    fn end_identifier(&self) -> Token {
        Token {
            token_type: TokenType::Identifier,
            literal: Some(format!(
                "_$__{}__end",
                self.value_tok.literal.as_ref().unwrap()
            )),
            start: self.value_tok.start.clone(),
            end: self.value_tok.end.clone(),
        }
    }

    fn step_identifier(&self) -> Token {
        Token {
            token_type: TokenType::Identifier,
            literal: Some(format!(
                "_$__{}__step",
                self.value_tok.literal.as_ref().unwrap()
            )),
            start: self.value_tok.start.clone(),
            end: self.value_tok.end.clone(),
        }
    }
}

impl AstNode for ForRangeLoopNode {
    fn setup(&mut self, ctx: MutRc<dyn Context>) -> Result<(), Error> {
        self.start.borrow_mut().setup(ctx.clone())?;
        self.end.borrow_mut().setup(ctx.clone())?;
        self.step.borrow_mut().setup(ctx.clone())?;

        self.value_decl_node = mut_rc(LocalVarNode {
            identifier: self.value_tok.clone(),
            value: self.start.clone(),
            mutable: false,
            type_annotation: None,
            start: self.value_tok.start.clone(),
            allow_anon_identifier: false,
        });
        self.value_decl_node.borrow_mut().setup(ctx.clone())?;

        if let Some(counter_tok) = &self.counter_tok {
            self.counter_decl_node = Some(mut_rc(LocalVarNode {
                identifier: counter_tok.clone(),
                value: mut_rc(IntNode {
                    value: 0,
                    position: (self.value_tok.start.clone(), self.value_tok.start.clone()),
                }),
                mutable: false,
                type_annotation: None,
                start: self.value_tok.start.clone(),
                allow_anon_identifier: true,
            }));
            self.counter_decl_node
                .as_mut()
                .unwrap()
                .borrow_mut()
                .setup(ctx.clone())?;
        }

        self.end_decl_node = mut_rc(LocalVarNode {
            identifier: self.end_identifier(),
            value: self.end.clone(),
            mutable: false,
            type_annotation: None,
            start: self.value_tok.start.clone(),
            allow_anon_identifier: true,
        });
        self.end_decl_node.borrow_mut().setup(ctx.clone())?;

        self.step_decl_node = mut_rc(LocalVarNode {
            identifier: self.step_identifier(),
            value: self.step.clone(),
            mutable: false,
            type_annotation: None,
            start: self.value_tok.start.clone(),
            allow_anon_identifier: true,
        });
        self.step_decl_node.borrow_mut().setup(ctx.clone())?;

        self.statements.borrow_mut().setup(ctx.clone())
    }
    fn type_check(&self, ctx: MutRc<dyn Context>) -> Result<TypeCheckRes, Error> {
        let TypeCheckRes { mut unknowns, .. } =
            self.value_decl_node.borrow_mut().type_check(ctx.clone())?;

        if let Some(counter_decl_node) = &self.counter_decl_node {
            unknowns += counter_decl_node
                .borrow_mut()
                .type_check(ctx.clone())?
                .unknowns;
        }

        unknowns += self
            .end_decl_node
            .borrow_mut()
            .type_check(ctx.clone())?
            .unknowns;

        unknowns += self
            .step_decl_node
            .borrow_mut()
            .type_check(ctx.clone())?
            .unknowns;

        let mut statements_tr = self.statements.borrow_mut().type_check(ctx.clone())?;
        statements_tr.unknowns += unknowns;
        Ok(statements_tr)
    }

    fn asm(&mut self, ctx: MutRc<dyn Context>) -> Result<String, Error> {
        let pre_body_lbl = ctx.borrow_mut().get_anon_label();
        let post_body_lbl = ctx.borrow_mut().get_anon_label();
        let post_loop_lbl = ctx.borrow_mut().get_anon_label();

        let value_dec_asm = self.value_decl_node.borrow_mut().asm(ctx.clone())?;
        let mut counter_dec_asm = "".to_string();
        if let Some(counter_decl_node) = &self.counter_decl_node {
            counter_dec_asm = counter_decl_node.borrow_mut().asm(ctx.clone())?;
        }
        let end_dec_asm = self.end_decl_node.borrow_mut().asm(ctx.clone())?;
        let step_dec_asm = self.step_decl_node.borrow_mut().asm(ctx.clone())?;

        ctx.borrow_mut().loop_labels_push(LoopLabels {
            post_body: post_body_lbl.clone(),
            post_loop: post_loop_lbl.clone(),
        });

        // loop label exists on loop label stack just inside loop body
        let body = self.statements.borrow_mut().asm(ctx.clone())?;

        ctx.borrow_mut().loop_labels_pop();

        let get_value_asm = ctx
            .borrow()
            .get_dec_from_id(&self.value_tok.literal.as_ref().unwrap().clone())
            .id;
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
                {value_dec_asm}
                {counter_dec_asm}
                {end_dec_asm}
                {step_dec_asm}
                {pre_body_lbl}:
                    mov rax, {get_value_asm}
                    cmp rax, {get_end_asm}
                    jge {post_loop_lbl}
                    {body}
                    {post_body_lbl}:
                    mov rax, {get_value_asm}
                    add rax, {get_step_asm}
                    mov {get_value_asm}, rax
                    {}
                    jmp {pre_body_lbl}
                {post_loop_lbl}:
            ",
            if let Some(counter_tok) = &self.counter_tok {
                let get_counter_asm = ctx
                    .borrow()
                    .get_dec_from_id(&counter_tok.literal.as_ref().unwrap().clone())
                    .id;
                format!("inc {get_counter_asm}")
            } else {
                "".to_string()
            },
        ))
    }

    fn pos(&self) -> Interval {
        self.position.clone()
    }
}
