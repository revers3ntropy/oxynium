use crate::parse::token::Token;
use crate::types::function::FnType;
use crate::types::generic::GenericType;
use crate::types::r#class::ClassType;
use crate::types::r#type::TypeType;
use crate::util::MutRc;
use std::collections::HashMap;
use std::fmt::Debug;

pub mod r#class;
pub mod function;
pub mod generic;
pub mod r#type;
pub mod unknown;

pub trait Type: Debug {
    fn is_ptr(&self) -> bool;
    fn str(&self) -> String;

    fn operator_signature(
        &self,
        _op: Token,
    ) -> Option<MutRc<FnType>> {
        None
    }
    fn contains(&self, other: MutRc<dyn Type>) -> bool;
    fn concrete(
        &self,
        generics_map: HashMap<String, MutRc<dyn Type>>,
        // prevent circular solidification
        already_concreted: &mut HashMap<
            String,
            MutRc<dyn Type>,
        >,
    ) -> MutRc<dyn Type>;

    fn as_fn(&self) -> Option<FnType> {
        None
    }
    fn as_class(&self) -> Option<ClassType> {
        None
    }
    fn as_type_type(&self) -> Option<TypeType> {
        None
    }
    fn as_generic(&self) -> Option<GenericType> {
        None
    }

    fn is_unknown(&self) -> bool {
        false
    }
}
