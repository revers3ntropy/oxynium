pub mod asm;
pub mod include;
pub mod include_asm_file;
pub mod unchecked_cast;

use crate::ast::AstNode;
use crate::context::Context;
use crate::error::Error;
use crate::util::MutRc;

pub trait Macro {
    fn resolve(&self, ctx: MutRc<dyn Context>) -> Result<MutRc<dyn AstNode>, Error>;
}
