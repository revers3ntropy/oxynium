use crate::ast::Node;
use crate::error::Error;
use crate::position::Position;
use crate::util::MutRc;

#[derive(Debug)]
pub struct ParseResults {
    pub node: Option<MutRc<dyn Node>>,
    pub error: Option<Error>,

    pub reverse_count: usize,
    pub last_registered_advance_count: usize,
    pub advance_count: usize,
}

impl ParseResults {
    pub fn new() -> ParseResults {
        ParseResults {
            node: None,
            error: None,
            reverse_count: 0,
            last_registered_advance_count: 0,
            advance_count: 0,
        }
    }

    pub fn register_advancement(&mut self) {
        self.advance_count = 1;
        self.last_registered_advance_count += 1;
    }

    pub fn register(&mut self, res: ParseResults) -> Option<MutRc<dyn Node>> {
        self.last_registered_advance_count = res.advance_count;
        self.advance_count += res.advance_count;
        if res.error.is_some() {
            self.error = res.error;
        }
        res.node
    }

    pub fn try_register(&mut self, res: ParseResults) -> Option<MutRc<dyn Node>> {
        if res.error.is_some() {
            self.reverse_count += res.advance_count;
            return None;
        }
        self.register(res)
    }

    pub fn success(&mut self, node: MutRc<dyn Node>) -> &ParseResults {
        self.node = Some(node);
        self
    }

    pub fn failure(
        &mut self,
        mut error: Error,
        start: Option<Position>,
        end: Option<Position>,
    ) -> &ParseResults {
        error.set_pos(
            start.unwrap_or(Position::unknown()),
            end.unwrap_or(Position::unknown()),
        );
        self.error = Some(error);
        self
    }
}
