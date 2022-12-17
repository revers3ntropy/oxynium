pub mod r#class;
pub mod function;

use crate::ast::types::function::FnType;
use crate::ast::types::r#class::ClassType;
use std::fmt::Debug;
use crate::util::MutRc;

pub trait Type: Debug {
    fn is_ptr(&self) -> bool;
    fn str(&self) -> String;

    fn contains(&self, other: MutRc<dyn Type>) -> bool;

    fn as_fn(&self) -> Option<FnType> {
        None
    }
    fn as_class(&self) -> Option<ClassType> {
        None
    }
}
