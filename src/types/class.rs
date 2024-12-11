use crate::ast::class_declaration::{method_id, operator_method_id};
use crate::error::Error;
use crate::parse::token::Token;
use crate::types::function::FnType;
use crate::types::Type;
use crate::util::{mut_rc, MutRc};
use std::collections::HashMap;

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
    pub methods: HashMap<String, MutRc<FnType>>,
    pub is_primitive: bool,
    pub generic_args: HashMap<String, MutRc<dyn Type>>,
    pub generic_params_order: Vec<Token>,
}

impl ClassType {
    pub fn field_type(&self, field: &str) -> Option<MutRc<dyn Type>> {
        self.fields.get(field).map(|f| f.type_.clone())
    }

    pub fn method_type(&self, method: &str) -> Option<MutRc<FnType>> {
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
        if self.generic_args.len() < 1 {
            self.name.clone()
        } else {
            format!(
                "{}<{}>",
                self.name,
                self.generic_params_order
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
            )
        }
    }

    fn operator_signature(&self, op: Token) -> Option<MutRc<FnType>> {
        self.methods
            .get(&operator_method_id(self.name.clone(), op))
            .map(|f| f.clone())
    }

    fn contains(&self, other: MutRc<dyn Type>) -> bool {
        if other.borrow().is_unknown() {
            return true;
        }
        if let Some(other) = other.borrow().as_class() {
            if other.id != self.id {
                return false;
            }
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
            true
        } else {
            false
        }
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
        if let Some(cached) = cache.get(&self.cache_id(&our_generics)) {
            return Ok(cached.clone());
        }

        if self.generic_params_order.len() < 1 {
            return Ok(mut_rc(self.clone()));
        }

        let res = mut_rc(ClassType {
            id: self.id,
            name: self.name.clone(),
            fields: HashMap::new(),
            methods: HashMap::new(),
            is_primitive: self.is_primitive,
            generic_args: HashMap::new(),
            generic_params_order: self.generic_params_order.clone(),
        });

        // outside of the loop to avoid borrowing issues
        let cache_id = self.cache_id(&our_generics);
        cache.insert(cache_id, res.clone());

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

        for field in self.fields.values() {
            res.borrow_mut().fields.insert(
                field.name.clone(),
                ClassFieldType {
                    name: field.name.clone(),
                    type_: field.type_.borrow().concrete(&our_generics, cache)?,
                    stack_offset: field.stack_offset,
                },
            );
        }

        // Concrete-ify any abstract method interfaces
        let methods = self.methods.clone();
        let method_names = methods.clone().into_keys();
        for name in method_names {
            let methods_clone = methods.clone();
            let method = methods_clone.get(name.as_str()).clone().unwrap();

            let new_method_type = method
                .borrow()
                .concrete(&our_generics, cache)?
                .borrow()
                .as_fn()
                .unwrap();

            res.borrow_mut()
                .methods
                .insert(name.clone(), mut_rc(new_method_type));
        }

        cache.remove(&self.cache_id(&our_generics));

        Ok(res)
    }

    fn cache_id(&self, generics: &HashMap<String, MutRc<dyn Type>>) -> String {
        if self.generic_params_order.len() < 1 {
            return self.id.to_string();
        }
        format!(
            "{}<{}>",
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

    fn as_class(&self) -> Option<ClassType> {
        Some(self.clone())
    }
}
