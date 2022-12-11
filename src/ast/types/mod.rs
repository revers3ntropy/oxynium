pub mod atomic;
pub mod function;

use std::fmt::Debug;
use std::rc::Rc;
use crate::ast::types::atomic::AtomicType;
use crate::ast::types::function::FnType;

pub trait Type: Debug {
    fn is_ptr(&self) -> bool;
    fn str(&self) -> String;

    fn contains(&self, other: Rc<dyn Type>) -> bool;

    fn as_atomic(&self) -> Option<AtomicType> { None }
    fn as_fn(&self) -> Option<FnType> { None }
}