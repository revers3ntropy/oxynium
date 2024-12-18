use crate::ast::AstNode;
use crate::error::Error;
use crate::parse::token::Token;
use crate::position::Interval;
use crate::types::unknown::UnknownType;
use crate::types::Type;
use crate::util::{mut_rc, MutRc};
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug)]
pub struct FnParamType {
    pub name: String,
    pub type_: MutRc<dyn Type>,
    pub default_value: Option<MutRc<dyn AstNode>>,
    pub position: Interval,
}
impl FnParamType {
    fn str(&self) -> String {
        if self.name == "" {
            self.type_.borrow().str()
        } else {
            format!("{}: {}", self.name, self.type_.borrow().str())
        }
    }
}

#[derive(Clone)]
pub struct FnType {
    pub id: usize,
    pub name: String,
    pub ret_type: MutRc<dyn Type>,
    pub parameters: Vec<FnParamType>,
    pub generic_args: HashMap<String, MutRc<dyn Type>>,
    pub generic_params_order: Vec<Token>,
}

impl Type for FnType {
    fn is_ptr(&self) -> bool {
        true
    }
    fn str(&self) -> String {
        format!(
            "Fn {}{}({}) {}",
            self.name,
            if self.generic_params_order.len() > 0 {
                format!(
                    "<{}>",
                    self.generic_params_order
                        .iter()
                        .map(|p| {
                            self.generic_args
                                .get(&p.clone().literal.unwrap())
                                .expect(&format!(
                                    "Generic argument '{}' not found",
                                    p.clone().literal.unwrap()
                                ))
                                .borrow()
                                .str()
                        })
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            } else {
                "".to_string()
            },
            self.parameters
                .iter()
                .map(|p| p.str())
                .collect::<Vec<String>>()
                .join(", "),
            self.ret_type.borrow().str()
        )
    }

    fn contains(&self, t: MutRc<dyn Type>) -> bool {
        if t.borrow().is_unknown() {
            return true;
        }
        if let Some(fn_type) = t.borrow().as_fn() {
            let required_args = self.parameters.iter().filter(|a| a.default_value.is_none());

            if !self.ret_type.borrow().contains(fn_type.ret_type.clone()) {
                return false;
            }

            if fn_type.parameters.len() < required_args.count()
                || fn_type.parameters.len() > self.parameters.len()
            {
                return false;
            }
            for i in 0..fn_type.parameters.len() {
                if !self.parameters[i]
                    .type_
                    .borrow()
                    .contains(fn_type.parameters[i].type_.clone())
                {
                    return false;
                }
            }
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
            return true;
        }
        false
    }

    fn concrete(
        &self,
        generics: &HashMap<String, MutRc<dyn Type>>,
        cache: &mut HashMap<String, MutRc<dyn Type>>,
    ) -> Result<MutRc<dyn Type>, Error> {
        let mut our_generics = generics.clone();
        for p in self.generic_params_order.iter() {
            let concrete_argument = self
                .generic_args
                .get(&p.clone().literal.unwrap())
                .unwrap()
                .borrow()
                .concrete(generics, cache)?;
            our_generics.insert(p.clone().literal.unwrap(), concrete_argument);
        }

        let res = mut_rc(FnType {
            id: self.id,
            name: self.name.clone(),
            ret_type: mut_rc(UnknownType {}),
            parameters: Vec::new(),
            generic_args: HashMap::new(),
            generic_params_order: self.generic_params_order.clone(),
        });

        for p in self.generic_params_order.iter() {
            res.borrow_mut().generic_args.insert(
                p.clone().literal.unwrap(),
                self.generic_args
                    .get(&p.clone().literal.unwrap())
                    .unwrap()
                    .borrow()
                    .concrete(&our_generics, cache)?,
            );
        }

        for param in &self.parameters {
            let type_ = param.type_.borrow().concrete(&our_generics, cache)?;
            res.borrow_mut().parameters.push(FnParamType {
                name: param.name.clone(),
                type_,
                default_value: param.default_value.clone(),
                position: param.position.clone(),
            });
        }

        let return_type = self.ret_type.borrow().concrete(&our_generics, cache)?;
        res.borrow_mut().ret_type = return_type;

        Ok(res)
    }

    fn cache_id(&self, generics: &HashMap<String, MutRc<dyn Type>>) -> String {
        if self.generic_params_order.len() < 1 {
            return format!("({})", self.str());
        }
        format!(
            "({}<{}>)",
            self.id,
            self.generic_params_order
                .iter()
                .map(|p| {
                    format!(
                        "{}:{}",
                        p.clone().literal.unwrap(),
                        self.generic_args
                            .get(&p.clone().literal.unwrap())
                            .unwrap()
                            .borrow()
                            .cache_id(generics)
                    )
                })
                .collect::<Vec<String>>()
                .join(", ")
        )
    }

    fn as_fn(&self) -> Option<FnType> {
        Some(self.clone())
    }
}

impl fmt::Debug for FnType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.str())
    }
}
