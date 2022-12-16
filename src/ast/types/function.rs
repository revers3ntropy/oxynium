use crate::ast::types::Type;
use crate::ast::Node;
use crate::util::MutRc;
use std::fmt;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct FnParamType {
    pub name: String,
    pub type_: Rc<dyn Type>,
    pub default_value: Option<MutRc<dyn Node>>,
}
impl FnParamType {
    fn str(&self) -> String {
        if self.name == "" {
            self.type_.str()
        } else {
            format!("{}: {}", self.name, self.type_.str())
        }
    }
}

#[derive(Clone)]
pub struct FnType {
    pub name: String,
    pub ret_type: Rc<dyn Type>,
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
            self.ret_type.str()
        )
    }

    fn contains(&self, t: Rc<dyn Type>) -> bool {
        if let Some(fn_type) = t.as_fn() {
            if fn_type.name != self.name {
                return false;
            }

            let required_args = self.parameters.iter().filter(|a| a.default_value.is_none());

            if fn_type.parameters.len() < required_args.count()
                || fn_type.parameters.len() > self.parameters.len()
            {
                return false;
            }
            for i in 0..fn_type.parameters.len() {
                if !self.parameters[i]
                    .type_
                    .contains(fn_type.parameters[i].type_.clone())
                {
                    return false;
                }
            }
            return true;
        }
        false
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
