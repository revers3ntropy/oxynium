use crate::ast::bin_op::BinOpNode;
use crate::ast::fn_call::FnCallNode;
use crate::ast::int::IntNode;
use crate::ast::local_var_decl::LocalVarNode;
use crate::ast::mutate_var::MutateVar;
use crate::ast::r#while::WhileLoopNode;
use crate::ast::statements::StatementsNode;
use crate::ast::symbol_access::SymbolAccessNode;
use crate::ast::{AstNode, TypeCheckRes};
use crate::context::Context;
use crate::error::Error;
use crate::parse::token::{Token, TokenType};
use crate::position::{Interval, Position};
use crate::util::{mut_rc, MutRc};

#[derive(Debug)]
pub struct ForLoopNode {
    pub start: Position,
    pub id_tok: Token,
    pub counter_tok: Token,
    pub value: MutRc<dyn AstNode>,
    pub statements: MutRc<dyn AstNode>,
    pub position: Interval,
    pub counter_var_assignment_node: MutRc<dyn AstNode>,
    pub local_var_assignment_node: MutRc<dyn AstNode>,
}

impl AstNode for ForLoopNode {
    fn setup(&mut self, ctx: MutRc<dyn Context>) -> Result<(), Error> {
        if self.counter_tok.literal.is_none() {
            self.counter_tok = Token::new(
                TokenType::Identifier,
                Some(format!(
                    "_$__{}__counter",
                    self.id_tok.literal.as_ref().unwrap()
                )),
                self.counter_tok.start.clone(),
                self.counter_tok.end.clone(),
            );
        }

        self.counter_var_assignment_node = mut_rc(LocalVarNode {
            identifier: self.counter_tok.clone(),
            value: mut_rc(IntNode {
                value: 0,
                position: (self.start.clone(), self.start.clone()),
            }),
            mutable: false,
            type_annotation: None,
            start: self.start.clone(),
            allow_anon_identifier: true,
        });

        self.local_var_assignment_node = mut_rc(LocalVarNode {
            identifier: self.id_tok.clone(),
            // args.at_raw(i)
            value: mut_rc(FnCallNode {
                object: Some(self.value.clone()),
                identifier: Token::new_unknown_pos(
                    TokenType::Identifier,
                    Some("at_raw".to_string()),
                ),
                args: vec![mut_rc(SymbolAccessNode {
                    identifier: self.counter_tok.clone(),
                })],
                generic_args: vec![],
                position: Position::unknown_interval(),
            }),
            mutable: false,
            type_annotation: None,
            start: self.id_tok.start.clone(),
            allow_anon_identifier: false,
        });

        self.local_var_assignment_node
            .borrow_mut()
            .setup(ctx.clone())?;
        self.counter_var_assignment_node
            .borrow_mut()
            .setup(ctx.clone())?;
        self.statements.borrow_mut().setup(ctx.clone())
    }

    fn type_check(&self, ctx: MutRc<dyn Context>) -> Result<TypeCheckRes, Error> {
        let TypeCheckRes {
            unknowns: counter_unknowns,
            ..
        } = self
            .counter_var_assignment_node
            .borrow_mut()
            .type_check(ctx.clone())?;
        let mut unknowns = counter_unknowns;

        let TypeCheckRes {
            unknowns: local_var_unknowns,
            ..
        } = self
            .local_var_assignment_node
            .borrow_mut()
            .type_check(ctx.clone())?;
        unknowns += local_var_unknowns;

        let mut statements_tr = self.statements.borrow_mut().type_check(ctx.clone())?;
        statements_tr.unknowns += unknowns;
        Ok(statements_tr)
    }

    fn asm(&mut self, ctx: MutRc<dyn Context>) -> Result<String, Error> {
        // while i < args.len() { ... }
        let mut while_loop_node = WhileLoopNode {
            condition: Some(mut_rc(BinOpNode {
                lhs: mut_rc(SymbolAccessNode {
                    identifier: self.counter_tok.clone(),
                }),
                operator: Token::new_unknown_pos(TokenType::LT, None),
                rhs: mut_rc(FnCallNode {
                    object: Some(self.value.clone()),
                    identifier: Token::new_unknown_pos(
                        TokenType::Identifier,
                        Some("len".to_string()),
                    ),
                    args: vec![],
                    generic_args: vec![],
                    position: Position::unknown_interval(),
                }),
            })),

            statements: mut_rc(StatementsNode {
                statements: vec![
                    // let arg = args.at_raw(i)
                    self.local_var_assignment_node.clone(),
                    self.statements.clone(),
                    // i = i + 1
                    mut_rc(MutateVar {
                        identifier: self.counter_tok.clone(),
                        value: mut_rc(BinOpNode {
                            lhs: mut_rc(SymbolAccessNode {
                                identifier: self.counter_tok.clone(),
                            }),
                            operator: Token::new_unknown_pos(TokenType::Plus, None),
                            rhs: mut_rc(IntNode {
                                value: 1,
                                position: Position::unknown_interval(),
                            }),
                        }),
                    }),
                ],
            }),
            position: self.position.clone(),
        };

        let counter_asm = self
            .counter_var_assignment_node
            .borrow_mut()
            .asm(ctx.clone())?;
        let while_loop_asm = while_loop_node.asm(ctx.clone())?;
        Ok(format!("{}{}", counter_asm, while_loop_asm))
    }

    fn pos(&self) -> Interval {
        self.position.clone()
    }
}
