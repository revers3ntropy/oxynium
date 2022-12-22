use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::{mismatched_types, type_error, Error};
use crate::parse::token::Token;
use crate::position::Interval;
use crate::util::MutRc;

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
