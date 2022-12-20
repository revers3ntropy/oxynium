use crate::ast::Node;
use crate::context::Context;
use crate::error::{syntax_error, Error};
use crate::position::Interval;
use crate::util::MutRc;

#[derive(Debug)]
pub struct ContinueNode {
    pub position: Interval,
}

impl Node for ContinueNode {
    fn asm(&mut self, ctx: MutRc<Context>) -> Result<String, Error> {
        let labels = ctx.borrow_mut().loop_label_peak();
        if labels.is_none() {
            return Err(syntax_error(
                "'continue' statement outside of loop".to_string(),
            ));
        }
        Ok(format!(
            "
            jmp {}
        ",
            labels.unwrap().0
        ))
    }
    fn pos(&self) -> Interval {
        self.position.clone()
    }
}
