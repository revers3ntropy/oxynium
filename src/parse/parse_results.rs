use crate::ast::node::Node;
use crate::error::Error;
use crate::position::Position;

pub(crate) struct ParseResults {
    pub node: Option<Box<dyn Node>>,
    pub error: Option<Error>,

    pub reverse_count: i64,
    pub last_registered_advance_count: i64,
    pub advance_count: i64,
}

impl ParseResults {
    pub fn new() -> ParseResults {
        ParseResults {
            node: None,
            error: None,
            reverse_count: 0,
            last_registered_advance_count: 0,
            advance_count: 0
        }
    }

    pub fn from_node(node: Box<dyn Node>) -> ParseResults {
        ParseResults {
            node: Some(node),
            error: None,
            reverse_count: 0,
            last_registered_advance_count: 0,
            advance_count: 0
        }
    }

    pub fn from_error(error: Error) -> ParseResults {
        ParseResults {
            node: None,
            error: Some(error),
            reverse_count: 0,
            last_registered_advance_count: 0,
            advance_count: 0
        }
    }

    pub fn register_advancement(&mut self) {
        self.advance_count += 1;
        self.advance_count = 0;
    }

    pub fn register(&mut self, res: ParseResults) -> Option<Box<dyn Node>> {
        self.last_registered_advance_count = res.advance_count;
        self.advance_count += res.advance_count;
        if res.error.is_some() {
            self.error = res.error;
        }
        res.node
    }

    pub fn try_register(&mut self, res: ParseResults) -> Option<Box<dyn Node>> {
        if res.error.is_some() {
            self.reverse_count += res.advance_count;
            return None;
        }
        self.register(res)
    }

    pub fn success(mut self, node: Box<dyn Node>) -> ParseResults {
        self.node = Some(node);
        self
    }

    pub fn failure(mut self, mut error: Error, start: Option<Position>, end: Option<Position>) -> ParseResults {
        error.set_pos(
            start.unwrap_or(Position::unknown()),
            end.unwrap_or(Position::unknown())
        );
        self.error = Some(error);
        self
    }
}