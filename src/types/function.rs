use crate::ast::Node;
use crate::position::Interval;
use crate::types::Type;
use crate::util::{new_mut_rc, MutRc};
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug)]
pub struct FnParamType {
    pub name: String,
    pub type_: MutRc<dyn Type>,
    pub default_value: Option<MutRc<dyn Node>>,
    pub position: Interval,
}
impl FnParamType {
    fn str(&self) -> String {
        if self.name == "" {
            self.type_.borrow().str()
        } else {
            format!(
                "{}: {}",
                self.name,
                self.type_.borrow().str()
            )
        }
    }
}

#[derive(Clone)]
pub struct FnType {
    pub name: String,
    pub ret_type: MutRc<dyn Type>,
    pub parameters: Vec<FnParamType>,
}

impl Type for FnType {
    fn is_ptr(&self) -> bool {
        true
    }
    fn str(&self) -> String {
        format!(
            "Fn {}({}): {}",
            self.name,
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
            let required_args = self
                .parameters
                .iter()
                .filter(|a| a.default_value.is_none());

            if fn_type.parameters.len()
                < required_args.count()
                || fn_type.parameters.len()
                    > self.parameters.len()
            {
                return false;
            }
            for i in 0..fn_type.parameters.len() {
                if !self.parameters[i]
                    .type_
                    .borrow()
                    .contains(
                        fn_type.parameters[i].type_.clone(),
                    )
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
        generics_map: HashMap<String, MutRc<dyn Type>>,
        already_concrete: &mut HashMap<
            String,
            MutRc<dyn Type>,
        >,
    ) -> MutRc<dyn Type> {
        if already_concrete
            .contains_key(&format!("{:p}", self))
        {
            return already_concrete
                .get(&format!("{:p}", self))
                .unwrap()
                .clone();
        }

        let mut parameters = Vec::new();
        for param in &self.parameters {
            let type_ = param.type_.borrow().concrete(
                generics_map.clone(),
                already_concrete,
            );
            parameters.push(FnParamType {
                name: param.name.clone(),
                type_,
                default_value: param.default_value.clone(),
                position: param.position.clone(),
            });
        }
        let res = new_mut_rc(FnType {
            name: self.name.clone(),
            ret_type: self
                .ret_type
                .borrow()
                .concrete(generics_map, already_concrete),
            parameters,
        });
        already_concrete
            .insert(format!("{:p}", self), res.clone());
        res
    }

    fn as_fn(&self) -> Option<FnType> {
        Some(self.clone())
    }
}

impl fmt::Debug for FnType {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "{}", self.str())
    }
}
