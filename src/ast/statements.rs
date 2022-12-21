use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::{type_error, Error};
use crate::position::Interval;
use crate::util::MutRc;

#[derive(Debug)]
pub struct StatementsNode {
    pub statements: Vec<MutRc<dyn Node>>,
    pub src: Vec<String>,
}

impl Node for StatementsNode {
    fn asm(&mut self, ctx: MutRc<Context>) -> Result<String, Error> {
        let mut asm = String::new();

        let mut i = 0;
        for statement in self.statements.iter_mut() {
            let stmt = statement.borrow_mut().asm(ctx.clone())?;
            if !stmt.is_empty() {
                asm.push_str("\n;- SRC: ");
                asm.push_str(self.src.get(i).unwrap());
                asm.push_str("\n");
                asm.push_str(&stmt.clone());
            }
            i += 1;
        }
        Ok(asm)
    }

    fn type_check(&self, ctx: MutRc<Context>) -> Result<TypeCheckRes, Error> {
        let mut ret_type = None;
        let mut unknowns = 0;

        for statement in self.statements.iter() {
            let t = statement.borrow().type_check(ctx.clone())?;
            unknowns += t.unknowns;

            if !t.is_returned {
                continue;
            }
            if ret_type.is_none() {
                ret_type = Some(t.t.clone());
            }
            if !ret_type.clone().unwrap().borrow().contains(t.t.clone()) {
                return Err(type_error(format!(
                    "Cannot return different types, expected `{}` found `{}`",
                    ret_type.unwrap().borrow().str(),
                    t.t.borrow().str()
                ))
                .set_interval(statement.borrow().pos()));
            }
        }

        if let Some(ret_type) = ret_type {
            return Ok(TypeCheckRes::from_return(ret_type, unknowns));
        }

        Ok(TypeCheckRes::from_ctx(&ctx, "Void", unknowns))
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
