use crate::ast::Node;
use crate::context::Context;
use crate::error::{syntax_error, Error};
use crate::position::Interval;
use crate::util::MutRc;

#[derive(Debug)]
pub struct BreakNode {
    pub position: Interval,
}

impl Node for BreakNode {
    fn asm(&mut self, ctx: MutRc<Context>) -> Result<String, Error> {
        let labels = ctx.borrow_mut().loop_label_peak();
        if labels.is_none() {
            return Err(syntax_error(
                "'break' statement outside of loop".to_string(),
            )
            .set_interval(self.pos()));
        }
        Ok(format!(
            "
            jmp {}
        ",
            labels.unwrap().1
        ))
    }
    fn pos(&self) -> Interval {
        self.position.clone()
    }
}
