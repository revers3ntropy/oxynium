use std::fmt::Debug;
use crate::context::Context;

pub(crate) trait Node: Debug {
    fn asm(&mut self, ctx: &mut Context) -> String;
}