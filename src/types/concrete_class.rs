use crate::error::Error;
use crate::parse::token::Token;
use crate::types::function::FnType;
use crate::types::generic_class::GenericClassType;
use crate::types::Type;
use crate::util::{mut_rc, MutRc};
use std::collections::HashMap;
use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct ConcreteClassType {
    pub class_type: GenericClassType,
    // expects generic arguments to be concrete already
    pub generic_args: HashMap<String, MutRc<dyn Type>>,
}

impl Type for ConcreteClassType {
    fn is_ptr(&self) -> bool {
        true
    }

    fn str(&self) -> String {
        format!(
            "{}<{}>",
            self.class_type.str(),
            self.generic_args
                .iter()
                .map(|(p, value)| {
                    format!(
                        "{}={}",
                        self.generic_args.get(p).unwrap().borrow().str(),
                        value.borrow().str()
                    )
                })
                .collect::<Vec<String>>()
                .join(", ")
        )
    }

    fn operator_signature(&self, op: Token) -> Option<MutRc<FnType>> {
        self.class_type.operator_signature(op)
    }

    fn contains(&self, other: MutRc<dyn Type>) -> bool {
        if other.borrow().is_unknown() {
            return true;
        }
        if let Some(other) = other.borrow().as_concrete_class() {
            for name in self.generic_args.keys() {
                if !self
                    .generic_args
                    .get(name)
                    .unwrap()
                    .borrow()
                    .contains(other.generic_args.get(name).unwrap().clone())
                {
                    return false;
                }
            }
            self.class_type.contains(mut_rc(other.class_type.clone()))
        } else {
            false
        }
    }

    fn concrete(
        &self,
        _generics: &HashMap<String, MutRc<dyn Type>>,
    ) -> Result<MutRc<dyn Type>, Error> {
        // or do we include the generic arguments we get from the caller?
        let mut our_generics = HashMap::new();
        for (k, v) in &self.generic_args {
            our_generics.insert(k.clone(), v.clone());
        }
        self.class_type.concrete(&our_generics)
    }

    fn cache_id(&self, generics: &HashMap<String, MutRc<dyn Type>>) -> String {
        format!(
            "{}<{}>",
            self.class_type.cache_id(generics),
            self.generic_args
                .iter()
                .map(|(k, value)| {
                    if value.borrow().as_generic().is_none() {
                        return value.borrow().cache_id(generics);
                    }
                    if !generics.contains_key(k) {
                        return format!("{}={}", k, k);
                    }
                    let concrete_type = generics.get(k).unwrap();
                    if format!("{:p}", self) == format!("{:p}", concrete_type.borrow().deref()) {
                        unreachable!()
                    }
                    format!("{}={}", k, concrete_type.borrow().cache_id(generics))
                })
                .collect::<Vec<String>>()
                .join(",")
        )
    }

    fn as_concrete_class(&self) -> Option<ConcreteClassType> {
        Some(self.clone())
    }
}
