use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::{mismatched_types, type_error, Error};
use crate::get_type;
use crate::parse::token::{Token, TokenType};
use crate::position::Interval;
use crate::types::unknown::UnknownType;
use crate::types::Type;
use crate::util::{new_mut_rc, MutRc};
use regex::Regex;

#[derive(Debug)]
pub struct BinOpNode {
    pub lhs: MutRc<dyn Node>,
    pub operator: Token,
    pub rhs: MutRc<dyn Node>,
}

impl Node for BinOpNode {
    fn asm(
        &mut self,
        ctx: MutRc<Context>,
    ) -> Result<String, Error> {
        let lhs = self
            .lhs
            .borrow_mut()
            .type_check(ctx.clone())?;
        let fn_signature = lhs
            .t
            .borrow()
            .operator_signature(self.operator.clone());

        let rhs = self
            .rhs
            .borrow_mut()
            .type_check(ctx.clone())?;
        if can_do_inline_bin_op(
            ctx.clone(),
            lhs.t.clone(),
            rhs.t.clone(),
            self.operator.clone(),
        ) {
            let lhs_asm =
                self.lhs.borrow_mut().asm(ctx.clone())?;
            let rhs_asm =
                self.rhs.borrow_mut().asm(ctx.clone())?;
            let res = do_inline_bin_op(
                lhs_asm,
                self.operator.clone(),
                rhs_asm,
            )?;
            return Ok(res);
        }

        Ok(format!(
            "
            {}
            {}
            call {}
            times 2 pop rcx
            push rax
        ",
            self.rhs.borrow_mut().asm(ctx.clone())?,
            self.lhs.borrow_mut().asm(ctx.clone())?,
            fn_signature.unwrap().borrow().name
        ))
    }

    fn type_check(
        &self,
        ctx: MutRc<Context>,
    ) -> Result<TypeCheckRes, Error> {
        let mut unknowns = 0;

        let lhs_tr = self
            .lhs
            .borrow_mut()
            .type_check(ctx.clone())?;
        let rhs_tr = self
            .rhs
            .borrow_mut()
            .type_check(ctx.clone())?;
        unknowns += lhs_tr.unknowns;
        unknowns += rhs_tr.unknowns;

        if lhs_tr.t.borrow().is_unknown() {
            return Ok(TypeCheckRes::unknown_and(unknowns));
        }

        let fn_signature = lhs_tr
            .t
            .borrow()
            .operator_signature(self.operator.clone());

        if fn_signature.is_none() {
            return Err(type_error(format!(
                "Cannot use operator `{}` on type `{}`",
                self.operator.str(),
                lhs_tr.t.borrow().str()
            ))
            .set_interval(self.pos()));
        }

        let fn_signature = fn_signature.unwrap();
        if !fn_signature.borrow().parameters[1]
            .type_
            .borrow()
            .contains(rhs_tr.t.clone())
        {
            return Err(mismatched_types(
                fn_signature.borrow().parameters[1]
                    .type_
                    .clone(),
                rhs_tr.t,
            )
            .set_interval(self.pos()));
        }

        let ret_type =
            fn_signature.borrow().ret_type.clone();

        Ok(TypeCheckRes::from(ret_type, unknowns))
    }
    fn pos(&self) -> Interval {
        (
            self.lhs.borrow_mut().pos().0,
            self.rhs.borrow_mut().pos().1,
        )
    }
}

fn can_do_inline_bin_op(
    ctx: MutRc<Context>,
    lhs_type: MutRc<dyn Type>,
    rhs_type: MutRc<dyn Type>,
    op: Token,
) -> bool {
    let operand_types = match op.token_type {
        TokenType::Percent
        | TokenType::Plus
        | TokenType::Sub
        | TokenType::Astrix
        | TokenType::FSlash
        | TokenType::DblEquals
        | TokenType::NotEquals
        | TokenType::GT
        | TokenType::LT
        | TokenType::GTE
        | TokenType::LTE => get_type!(ctx, "Int"),
        _ => get_type!(ctx, "Bool"),
    };
    if !operand_types.borrow().contains(lhs_type) {
        return false;
    }
    if !operand_types.borrow().contains(rhs_type) {
        return false;
    }
    true
}

fn do_inline_bin_op(
    lhs: String,
    op: Token,
    rhs: String,
) -> Result<String, Error> {
    if vec![TokenType::Plus, TokenType::Sub]
        .contains(&op.token_type)
    {
        let add_0_re =
            Regex::new(r"^mov rax, 0\n +push rax$")
                .unwrap();
        if add_0_re.is_match(lhs.trim()) {
            if op.token_type == TokenType::Plus {
                return Ok(rhs);
            }
            return Ok(format!(
                "
                    {rhs}
                    neg qword [rsp]
                "
            ));
        }
        if add_0_re.is_match(rhs.trim()) {
            return Ok(lhs);
        }
        let add_1_re =
            Regex::new(r"^mov rax, 1\n +push rax$")
                .unwrap();
        let inc_operator = match op.token_type {
            TokenType::Plus => "inc",
            TokenType::Sub => "dec",
            _ => unreachable!(),
        };
        if add_1_re.is_match(lhs.trim()) {
            if op.token_type == TokenType::Plus {
                return Ok(format!(
                    "
                        {rhs}
                        {inc_operator} qword [rsp]
                    "
                ));
            }
            return Ok(format!(
                "
                    {rhs}
                    neg qword [rsp]
                    inc qword [rsp]
                "
            ));
        }
        if add_1_re.is_match(rhs.trim()) {
            return Ok(format!(
                "
                    {lhs}
                    {inc_operator} qword [rsp]
                "
            ));
        }
    }

    match op.token_type {
        TokenType::Plus
        | TokenType::Sub
        | TokenType::And
        | TokenType::Or => {
            Ok(format!(
                "
                    {}
                    {}
                    pop rax
                    pop rbx
                    {} rax, rbx
                    push rax
                ",
                rhs,
                lhs,
                match op.token_type {
                    TokenType::Plus => "add",
                    TokenType::Sub => "sub",
                    TokenType::And => "and",
                    _ => "or",
                }
            ))
        },
        TokenType::Astrix | TokenType::FSlash => {
            Ok(format!(
                "
            {}
            {}
            pop rax
            pop rbx
            cqo ; extend rax to rdx:rax
            {} rbx
            push rax
        ",
                rhs,
                lhs,
                match op.token_type {
                    TokenType::Astrix => "imul",
                    _ => "idiv",
                }
            ))
        },
        TokenType::Percent => {
            Ok(format!(
                "
                        {}
                        {}
                        pop rax
                        pop rbx
                        cqo ; extend rax to rdx:rax
                        idiv rbx
                        push rdx
                    ",
                rhs,
                lhs,
            ))
        },
        TokenType::GT
        | TokenType::LT
        | TokenType::LTE
        | TokenType::GTE
        | TokenType::DblEquals
        | TokenType::NotEquals => {
            Ok(format!(
                "
                        {}
                        {}
                        pop rcx ; lhs
                        pop rdx ; rhs
                        xor rax, rax     ; al is first byte of rax,
                        cmp rcx, rdx
                        {} al          ; so clear rax and put into al
                        push rax
                ",
                rhs,
                lhs,
                match op.token_type {
                    TokenType::DblEquals => "sete",
                    TokenType::NotEquals => "setne",
                    TokenType::GT => "setg",
                    TokenType::LT => "setl",
                    TokenType::GTE => "setge",
                    _ => "setle",
                }
            ))
        },
        _ => unreachable!()
    }
}
