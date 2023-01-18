pub mod asm;

use crate::ast::Node;
use crate::context::Context;
use crate::error::Error;
use crate::util::MutRc;

pub trait Macro {
    fn resolve(
        &self,
        ctx: MutRc<Context>,
    ) -> Result<MutRc<dyn Node>, Error>;
}
