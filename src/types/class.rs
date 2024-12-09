use crate::ast::class_declaration::{method_id, operator_method_id};
use crate::ast::CallableType;
use crate::error::Error;
use crate::parse::token::Token;
use crate::types::function::FnType;
use crate::types::Type;
use crate::util::{mut_rc, MutRc};
use std::collections::HashMap;
use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct ClassFieldType {
    pub name: String,
    pub type_: MutRc<dyn Type>,
    pub stack_offset: usize,
}

#[derive(Clone, Debug)]
pub struct ClassType {
    pub id: usize,
    pub name: String,
    pub fields: HashMap<String, ClassFieldType>,
    pub methods: HashMap<String, MutRc<CallableType>>,
    pub is_primitive: bool,
}

impl ClassType {
    pub fn field_type(&self, field: &str) -> Option<MutRc<dyn Type>> {
        self.fields.get(field).map(|f| f.type_.clone())
    }

    pub fn method_type(&self, method: &str) -> Option<MutRc<CallableType>> {
        self.methods
            .get(&method_id(self.name.clone(), method.to_string()))
            .map(|f| f.clone())
    }

    pub fn field_offset(&self, field: String) -> usize {
        self.fields.get(&field).unwrap().stack_offset
    }
}

impl Type for ClassType {
    fn is_ptr(&self) -> bool {
        true
    }

    fn str(&self) -> String {
        self.name.clone()
    }

    fn operator_signature(&self, op: Token) -> Option<MutRc<FnType>> {
        self.methods
            .get(&operator_method_id(self.name.clone(), op))
            .map(|f| f.clone())
            .map(|f| match f.borrow().deref() {
                CallableType::Fn(f) => mut_rc(f.clone()),
                // should not be able to declare operator overload methods as generic
                _ => unreachable!(),
            })
    }

    fn contains(&self, other: MutRc<dyn Type>) -> bool {
        if other.borrow().is_unknown() {
            return true;
        }
        if let Some(other) = other.borrow().as_class() {
            if other.id != self.id {
                return false;
            }
            true
        } else {
            false
        }
    }

    fn concrete(
        &self,
        _generics: &HashMap<String, MutRc<dyn Type>>,
    ) -> Result<MutRc<dyn Type>, Error> {
        Ok(mut_rc(self.clone()))
    }

    fn cache_id(&self, _generics: &HashMap<String, MutRc<dyn Type>>) -> String {
        self.id.to_string()
    }

    fn as_class(&self) -> Option<ClassType> {
        Some(self.clone())
    }
}
