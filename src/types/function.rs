use crate::ast::Node;
use crate::context::Context;
use crate::error::Error;
use crate::position::Interval;
use crate::types::unknown::UnknownType;
use crate::types::Type;
use crate::util::{new_mut_rc, MutRc};
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
    pub id: usize,
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
        ctx: MutRc<Context>,
    ) -> Result<MutRc<dyn Type>, Error> {
        if let Some(cached) =
            ctx.borrow().concrete_type_cache_get(
                self.cache_id(ctx.clone()),
            )
        {
            return Ok(cached);
        }

        let res = new_mut_rc(FnType {
            id: self.id,
            name: self.name.clone(),
            ret_type: new_mut_rc(UnknownType {}),
            parameters: Vec::new(),
        });

        // outside of the loop to avoid borrowing issues
        let cache_id = self.cache_id(ctx.clone());
        ctx.borrow_mut()
            .concrete_type_cache_set(cache_id, res.clone());

        res.borrow_mut().ret_type =
            self.ret_type.borrow().concrete(ctx.clone())?;

        for param in &self.parameters {
            let type_ = param
                .type_
                .borrow()
                .concrete(ctx.clone())?;
            res.borrow_mut().parameters.push(FnParamType {
                name: param.name.clone(),
                type_,
                default_value: param.default_value.clone(),
                position: param.position.clone(),
            });
        }

        Ok(res)
    }

    fn cache_id(&self, _ctx: MutRc<Context>) -> String {
        format!("({})", self.str())
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
