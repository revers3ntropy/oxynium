use crate::error::Error;
use crate::parse::token::Token;
use crate::types::function::{FnParamType, FnType};
use crate::types::unknown::UnknownType;
use crate::types::Type;
use crate::util::{mut_rc, MutRc};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct GenericFnType {
    pub fn_type: FnType,
    // guaranteed to have elements
    pub generic_params_order: Vec<Token>,
}

impl Type for GenericFnType {
    fn is_ptr(&self) -> bool {
        true
    }
    fn str(&self) -> String {
        format!(
            "Fn {}{}({}) {}",
            self.fn_type.name,
            format!(
                "<{}>",
                self.generic_params_order
                    .iter()
                    .map(|p| { p.str() })
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            self.fn_type
                .parameters
                .iter()
                .map(|p| p.str())
                .collect::<Vec<String>>()
                .join(", "),
            self.fn_type.return_type.borrow().str()
        )
    }

    fn contains(&self, t: MutRc<dyn Type>) -> bool {
        if t.borrow().is_unknown() {
            return true;
        }
        if let Some(fn_type) = t.borrow().as_generic_fn() {
            // only check the number of generic args
            // so that def<T> is assignable to def<U>
            if self.generic_params_order.len() != fn_type.generic_params_order.len() {
                return false;
            }

            return self.fn_type.contains(mut_rc(fn_type.fn_type));
        }
        false
    }

    fn concrete(
        &self,
        generics: &HashMap<String, MutRc<dyn Type>>,
    ) -> Result<MutRc<dyn Type>, Error> {
        let res = mut_rc(FnType {
            id: self.fn_type.id,
            name: self.fn_type.name.clone(),
            return_type: mut_rc(UnknownType {}),
            parameters: Vec::new(),
        });

        for param in &self.fn_type.parameters {
            let type_ = param.type_.borrow().concrete(generics)?;
            res.borrow_mut().parameters.push(FnParamType {
                name: param.name.clone(),
                type_,
                default_value: param.default_value.clone(),
                position: param.position.clone(),
            });
        }

        let return_type = self.fn_type.return_type.borrow().concrete(generics)?;
        res.borrow_mut().return_type = return_type;

        Ok(res)
    }

    fn cache_id(&self, _generics: &HashMap<String, MutRc<dyn Type>>) -> String {
        unreachable!()
    }

    fn as_generic_fn(&self) -> Option<GenericFnType> {
        Some(self.clone())
    }
}
