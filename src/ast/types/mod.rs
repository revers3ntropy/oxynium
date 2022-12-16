pub mod atomic;
pub mod function;
pub mod r#struct;

use crate::ast::types::atomic::AtomicType;
use crate::ast::types::function::FnType;
use crate::ast::types::r#struct::StructType;
use std::fmt::Debug;
use std::rc::Rc;

pub trait Type: Debug {
    fn is_ptr(&self) -> bool;
    fn str(&self) -> String;

    fn contains(&self, other: Rc<dyn Type>) -> bool;

    fn as_atomic(&self) -> Option<AtomicType> {
        None
    }
    fn as_fn(&self) -> Option<FnType> {
        None
    }
    fn as_struct(&self) -> Option<StructType> {
        None
    }
}
