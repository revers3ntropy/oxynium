use crate::ast::CallableType;
use crate::context::Context;
use crate::error::Error;
use crate::parse::token::Token;
use crate::types::class::{ClassFieldType, ClassType};
use crate::types::function::FnType;
use crate::types::Type;
use crate::util::{mut_rc, MutRc};
use std::collections::HashMap;
use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct GenericClassType {
    pub class_type: ClassType,
    pub generic_params_order: Vec<Token>,
    pub parent_ctx: MutRc<dyn Context>,
}

impl Type for GenericClassType {
    fn is_ptr(&self) -> bool {
        true
    }

    fn str(&self) -> String {
        format!(
            "{}<{}>",
            self.class_type.str(),
            self.generic_params_order
                .iter()
                .map(|p| { p.str() })
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
        if let Some(other) = other.borrow().as_generic_class() {
            if self.generic_params_order.len() != other.generic_params_order.len() {
                return false;
            }
            self.class_type.contains(mut_rc(other.class_type.clone()))
        } else {
            false
        }
    }

    fn concrete(
        &self,
        generics: &HashMap<String, MutRc<dyn Type>>,
    ) -> Result<MutRc<dyn Type>, Error> {
        if let Some(cached) = self
            .parent_ctx
            .borrow()
            .concrete_type_cache_get(self.cache_id(&generics))
        {
            return Ok(cached);
        }

        let res = mut_rc(ClassType {
            id: self.class_type.id,
            name: self.class_type.name.clone(),
            fields: HashMap::new(),
            methods: HashMap::new(),
            is_primitive: self.class_type.is_primitive,
        });

        // outside of the loop to avoid borrowing issues
        let cache_id = self.cache_id(&generics);
        self.parent_ctx
            .borrow_mut()
            .concrete_type_cache_set(cache_id, res.clone());

        for field in self.class_type.fields.values() {
            res.borrow_mut().fields.insert(
                field.name.clone(),
                ClassFieldType {
                    name: field.name.clone(),
                    type_: field.type_.borrow().concrete(&generics)?,
                    stack_offset: field.stack_offset,
                },
            );
        }

        // Concrete-ify any abstract method interfaces
        let methods = self.class_type.methods.clone();
        let method_names = methods.clone().into_keys();
        for name in method_names {
            let method = methods.get(name.as_str()).clone().unwrap();

            let new_method_type = method
                .borrow()
                .concrete(&generics)?
                .borrow()
                .as_fn()
                .unwrap();

            res.borrow_mut()
                .methods
                .insert(name.clone(), mut_rc(CallableType::Fn(new_method_type)));
        }

        let cache_id = self.cache_id(&generics);
        self.parent_ctx
            .borrow_mut()
            .concrete_type_cache_remove(&cache_id);

        Ok(res)
    }

    fn cache_id(&self, generics: &HashMap<String, MutRc<dyn Type>>) -> String {
        format!(
            "{}<{}>",
            self.class_type.cache_id(generics),
            self.generic_params_order
                .iter()
                .map(|k| { k.literal.clone().unwrap() })
                .map(|k| {
                    if !generics.contains_key(&k) {
                        return format!("{}:{}", k, k);
                    }
                    let value = generics.get(&k).unwrap();
                    if format!("{:p}", self) == format!("{:p}", value.borrow().deref()) {
                        unreachable!()
                    }
                    format!("{}:{}", k, value.borrow().cache_id(generics))
                })
                .collect::<Vec<String>>()
                .join(",")
        )
    }

    fn as_generic_class(&self) -> Option<GenericClassType> {
        Some(self.clone())
    }
}
