use crate::ast::AstNode;
use crate::error::Error;
use crate::position::Position;
use crate::util::MutRc;

#[derive(Debug)]
pub struct ParseResults {
    pub node: Option<MutRc<dyn AstNode>>,
    pub error: Option<Error>,

    pub last_registered_advance_count: usize,
    pub advance_count: usize,
}

impl ParseResults {
    pub fn new() -> ParseResults {
        ParseResults {
            node: None,
            error: None,
            last_registered_advance_count: 0,
            advance_count: 0,
        }
    }

    pub fn register_advancement(&mut self) {
        self.advance_count = 1;
        self.last_registered_advance_count += 1;
    }

    pub fn register(&mut self, res: ParseResults) -> Option<MutRc<dyn AstNode>> {
        self.last_registered_advance_count = res.advance_count;
        self.advance_count += res.advance_count;
        if res.error.is_some() {
            self.error = res.error;
        }
        res.node
    }

    pub fn register_result<T>(&mut self, res: Result<T, Error>) -> Option<T> {
        match res {
            Ok(node) => Some(node),
            Err(err) => {
                self.error = Some(err);
                None
            }
        }
    }

    pub fn success(&mut self, node: MutRc<dyn AstNode>) -> &ParseResults {
        self.node = Some(node);
        self
    }

    pub fn failure(
        &mut self,
        mut error: Error,
        start: Option<Position>,
        end: Option<Position>,
    ) -> &ParseResults {
        if start.is_some() && error.start.is_unknown() {
            error.start = start.unwrap();
        }
        if end.is_some() && error.end.is_unknown() {
            error.end = end.unwrap();
        }
        self.error = Some(error);
        self
    }
}
