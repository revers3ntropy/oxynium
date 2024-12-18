use crate::ast::AstNode;
use crate::context::Context;
use crate::error::{syntax_error, Error};
use crate::position::Interval;
use crate::util::MutRc;

#[derive(Debug)]
pub struct BreakNode {
    pub position: Interval,
}

impl AstNode for BreakNode {
    fn asm(&mut self, ctx: MutRc<dyn Context>) -> Result<String, Error> {
        let labels = ctx.borrow().loop_label_peak();
        if labels.is_none() {
            return Err(
                syntax_error("'break' statement outside of loop".to_string())
                    .set_interval(self.pos()),
            );
        }
        Ok(format!(
            "
                jmp {}
            ",
            labels.unwrap().post_loop
        ))
    }

    fn pos(&self) -> Interval {
        self.position.clone()
    }
}
