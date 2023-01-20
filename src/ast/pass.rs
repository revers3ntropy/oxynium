use crate::ast::AstNode;
use crate::position::Interval;

#[derive(Debug)]
pub struct PassNode {
    pub position: Interval,
}

impl AstNode for PassNode {
    fn pos(&self) -> Interval {
        self.position.clone()
    }
}
