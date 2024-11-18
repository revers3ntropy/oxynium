use crate::ast::{AstNode, TypeCheckRes};
use crate::context::Context;
use crate::error::{mismatched_types, type_error, Error};
use crate::get_type;
use crate::parse::token::{Token, TokenType};
use crate::position::Interval;
use crate::post_process::optimise::o1;
use crate::types::unknown::UnknownType;
use crate::types::Type;
use crate::util::{mut_rc, MutRc};
use regex::Regex;

#[derive(Debug)]
pub struct BinOpNode {
    pub lhs: MutRc<dyn AstNode>,
    pub operator: Token,
    pub rhs: MutRc<dyn AstNode>,
}

impl AstNode for BinOpNode {
    fn setup(&mut self, ctx: MutRc<dyn Context>) -> Result<(), Error> {
        self.lhs.borrow_mut().setup(ctx.clone())?;
        self.rhs.borrow_mut().setup(ctx.clone())
    }

    fn type_check(&self, ctx: MutRc<dyn Context>) -> Result<TypeCheckRes, Error> {
        let mut unknowns = 0;

        let lhs_tr = self.lhs.borrow().type_check(ctx.clone())?;
        let rhs_tr = self.rhs.borrow().type_check(ctx.clone())?;
        unknowns += lhs_tr.unknowns;
        unknowns += rhs_tr.unknowns;

        if lhs_tr.t.borrow().is_unknown() {
            if ctx.borrow().throw_on_unknowns() {
                return Err(type_error(
                    "unknown type on left hand side of binary operator".to_string(),
                )
                .set_interval(self.lhs.borrow().pos()));
            }
            return Ok(TypeCheckRes::unknown_and(unknowns));
        }

        let fn_signature = lhs_tr.t.borrow().operator_signature(self.operator.clone());

        if fn_signature.is_none() {
            return Err(type_error(format!(
                "cannot use operator `{}` on type `{}`",
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
                fn_signature.borrow().parameters[1].type_.clone(),
                rhs_tr.t,
            )
            .set_interval(self.pos()));
        }

        let ret_type = fn_signature.borrow().ret_type.clone();

        Ok(TypeCheckRes::from(ret_type, unknowns))
    }

    fn asm(&mut self, ctx: MutRc<dyn Context>) -> Result<String, Error> {
        let lhs = self.lhs.borrow().type_check(ctx.clone())?;
        let fn_signature = lhs.t.borrow().operator_signature(self.operator.clone());

        let rhs = self.rhs.borrow().type_check(ctx.clone())?;
        if can_do_inline(
            ctx.clone(),
            lhs.t.clone(),
            rhs.t.clone(),
            self.operator.clone(),
        ) {
            let lhs_asm = self.lhs.borrow_mut().asm(ctx.clone())?;
            let rhs_asm = self.rhs.borrow_mut().asm(ctx.clone())?;
            let res = do_inline(lhs_asm, self.operator.clone(), rhs_asm, ctx.clone());
            if res.is_err() {
                return Err(res.err().unwrap().set_interval(self.pos()));
            }
            return res;
        }

        Ok(format!(
            "
            {}
            {}
            call {}
            add rsp, 16
            push rax
        ",
            self.rhs.borrow_mut().asm(ctx.clone())?,
            self.lhs.borrow_mut().asm(ctx.clone())?,
            fn_signature.unwrap().borrow().name
        ))
    }
    fn pos(&self) -> Interval {
        (self.lhs.borrow().pos().0, self.rhs.borrow().pos().1)
    }
}

fn can_do_inline(
    ctx: MutRc<dyn Context>,
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
    operand_types.borrow().contains(lhs_type) && operand_types.borrow().contains(rhs_type)
}

fn do_inline(
    lhs: String,
    op: Token,
    rhs: String,
    ctx: MutRc<dyn Context>,
) -> Result<String, Error> {
    let push_0_regex = Regex::new(r"^mov rax, 0\n +push rax$").unwrap();
    let push_1_regex = Regex::new(r"^mov rax, 1\n +push rax$").unwrap();
    let o1_res = o1("constant-folding", &ctx.borrow().get_cli_args(), &|| {
        if vec![TokenType::Plus, TokenType::Sub].contains(&op.token_type) {
            if push_0_regex.is_match(lhs.trim()) {
                if op.token_type == TokenType::Plus {
                    return Some(rhs.clone());
                }
                return Some(format!(
                    "
                        {rhs}
                        pop rax
                        neg rax
                        push rax
                    "
                ));
            }
            if push_0_regex.is_match(rhs.trim()) {
                return Some(lhs.clone());
            }

            let inc_operator = match op.token_type {
                TokenType::Plus => "inc",
                TokenType::Sub => "dec",
                _ => unreachable!(),
            };
            if push_1_regex.is_match(lhs.trim()) {
                if op.token_type == TokenType::Plus {
                    return Some(format!(
                        "
                            {rhs}
                            {inc_operator} qword [rsp]
                        "
                    ));
                }
                return Some(format!(
                    "
                        {rhs}
                        pop rax
                        neg rax
                        inc rax
                        push rax
                    "
                ));
            }
            if push_1_regex.is_match(rhs.trim()) {
                return Some(format!(
                    "
                        {lhs}
                        pop rax
                        {inc_operator} rax
                        push rax
                    "
                ));
            }
        }

        if op.token_type == TokenType::Astrix {
            if push_1_regex.is_match(lhs.trim()) {
                return Some(rhs.clone());
            }
            if push_1_regex.is_match(rhs.trim()) {
                return Some(lhs.clone());
            }
            if push_0_regex.is_match(lhs.trim()) {
                return Some(lhs.clone());
            }
            if push_0_regex.is_match(rhs.trim()) {
                return Some(rhs.clone());
            }
        }
        if op.token_type == TokenType::FSlash {
            if push_1_regex.is_match(rhs.trim()) {
                return Some(lhs.clone());
            }
            if push_0_regex.is_match(lhs.trim()) && !push_0_regex.is_match(rhs.trim()) {
                return Some(lhs.clone());
            }
        }
        return None;
    });

    if let Some(o1_res) = o1_res {
        if let Some(o1_res) = o1_res {
            return Ok(o1_res);
        }
    }

    if op.token_type == TokenType::FSlash && push_0_regex.is_match(rhs.trim()) {
        return Err(type_error("Nice try".to_string()));
    }

    match op.token_type {
        TokenType::Plus | TokenType::Sub | TokenType::And | TokenType::Or => Ok(format!(
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
        )),
        TokenType::Astrix | TokenType::FSlash => Ok(format!(
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
        )),
        TokenType::Percent => Ok(format!(
            "
                        {}
                        {}
                        pop rax
                        pop rbx
                        cqo ; extend rax to rdx:rax
                        idiv rbx
                        push rdx
                    ",
            rhs, lhs,
        )),
        TokenType::GT
        | TokenType::LT
        | TokenType::LTE
        | TokenType::GTE
        | TokenType::DblEquals
        | TokenType::NotEquals => Ok(format!(
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
        )),
        _ => unreachable!(),
    }
}
