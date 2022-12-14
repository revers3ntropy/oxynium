use crate::ast::Node;
use crate::position::Interval;

#[derive(Debug)]
pub struct PassNode {
    pub position: Interval,
}

impl Node for PassNode {
    fn pos(&self) -> Interval {
        self.position.clone()
    }
}
