use crate::ast::{AstNode, TypeCheckRes};
use crate::context::Context;
use crate::error::{syntax_error, type_error, Error};
use crate::position::Interval;
use crate::util::MutRc;

#[derive(Debug)]
pub struct StatementsNode {
    pub statements: Vec<MutRc<dyn AstNode>>,
}

impl AstNode for StatementsNode {
    fn setup(&mut self, ctx: MutRc<dyn Context>) -> Result<(), Error> {
        for statement in self.statements.iter_mut() {
            statement.borrow_mut().setup(ctx.clone())?;
        }
        Ok(())
    }
    fn type_check(&self, ctx: MutRc<dyn Context>) -> Result<TypeCheckRes, Error> {
        let mut ret_type = None;
        let mut always_returns = false;
        let mut unknowns = 0;

        for statement in self.statements.iter() {
            if always_returns {
                return Err(syntax_error("unreachable code".to_string())
                    .set_interval(statement.borrow().pos()));
            }
            let t = statement.borrow().type_check(ctx.clone())?;
            unknowns += t.unknowns;

            if !t.is_returned {
                continue;
            }
            if t.always_returns {
                always_returns = true;
            }
            if ret_type.is_none() {
                ret_type = Some(t.t.clone());
            }
            if !ret_type.clone().unwrap().borrow().contains(t.t.clone()) {
                return Err(type_error(format!(
                    "cannot return different types, expected `{}` found `{}`",
                    ret_type.unwrap().borrow().str(),
                    t.t.borrow().str()
                ))
                .set_interval(statement.borrow().pos()));
            }
        }

        if let Some(ret_type) = ret_type {
            return Ok(TypeCheckRes::returns(always_returns, ret_type, unknowns));
        }

        Ok(TypeCheckRes::from_type_in_ctx(&ctx, "Void", unknowns, true))
    }

    fn asm(&mut self, ctx: MutRc<dyn Context>) -> Result<String, Error> {
        let mut asm = String::new();

        for statement in self.statements.iter_mut() {
            let stmt = statement.borrow_mut().asm(ctx.clone())?;
            if !stmt.is_empty() {
                asm.push_str("\n");
                asm.push_str(&stmt.clone());
            }
        }
        Ok(asm)
    }

    fn pos(&self) -> Interval {
        (
            self.statements[0].borrow_mut().pos().0,
            self.statements[self.statements.len() - 1]
                .borrow_mut()
                .pos()
                .1,
        )
    }
}
