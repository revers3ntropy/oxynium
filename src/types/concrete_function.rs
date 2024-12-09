use crate::error::Error;
use crate::types::generic_function::GenericFnType;
use crate::types::Type;
use crate::util::{mut_rc, MutRc};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ConcreteFnType {
    pub fn_type: GenericFnType,
    pub generic_args: HashMap<String, MutRc<dyn Type>>,
}

impl Type for ConcreteFnType {
    fn is_ptr(&self) -> bool {
        true
    }
    fn str(&self) -> String {
        format!(
            "Fn {}{}({}) {}",
            self.fn_type.fn_type.name,
            format!(
                "<{}>",
                self.fn_type
                    .generic_params_order
                    .iter()
                    .map(|p| {
                        self.generic_args
                            .get(&p.clone().literal.unwrap())
                            .unwrap()
                            .borrow()
                            .str()
                    })
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            self.fn_type
                .fn_type
                .parameters
                .iter()
                .map(|p| p.str())
                .collect::<Vec<String>>()
                .join(", "),
            self.fn_type.fn_type.return_type.borrow().str()
        )
    }

    fn contains(&self, t: MutRc<dyn Type>) -> bool {
        if t.borrow().is_unknown() {
            return true;
        }
        if let Some(fn_type) = t.borrow().as_concrete_fn() {
            for name in self.generic_args.keys() {
                if !self
                    .generic_args
                    .get(name)
                    .unwrap()
                    .borrow()
                    .contains(fn_type.generic_args.get(name).unwrap().clone())
                {
                    return false;
                }
            }
            return self.fn_type.contains(mut_rc(fn_type.fn_type));
        }
        false
    }

    fn concrete(
        &self,
        generics: &HashMap<String, MutRc<dyn Type>>,
    ) -> Result<MutRc<dyn Type>, Error> {
        let mut our_generics = generics.clone();
        our_generics.extend(self.generic_args.clone());
        self.fn_type.concrete(&our_generics)
    }

    fn cache_id(&self, _generics: &HashMap<String, MutRc<dyn Type>>) -> String {
        unreachable!()
    }

    fn as_concrete_fn(&self) -> Option<ConcreteFnType> {
        Some(self.clone())
    }
}
