use crate::ast::class_declaration::{method_id, operator_method_id};
use crate::ast::ANON_PREFIX;
use crate::error::Error;
use crate::parse::token::Token;
use crate::types::function::{FnParamType, FnType};
use crate::types::generic::GenericType;
use crate::types::unknown::UnknownType;
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

    fn rename_generic_param_for_method(param_name: String) -> String {
        format!("{ANON_PREFIX}{param_name}")
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
        if self.generic_params_order.len() < 1 {
            return Ok(mut_rc(self.clone()));
        }

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
        let cache_id = self.cache_id(&our_generics);
        if let Some(cached) = cache.get(&cache_id) {
            return Ok(cached.clone());
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

        cache.insert(cache_id.clone(), res.clone());

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
        for (name, method_type) in self.methods.clone() {
            // skip static methods
            // TODO why does removing this make performance very bad??
            if let Some(first_param) = method_type.borrow().parameters.first() {
                if first_param.name != "self" {
                    continue;
                }
            } else {
                continue;
            }
            let method_type = method_type.borrow();
            let mut new_method_type = FnType {
                id: method_type.id,
                name: method_type.name.clone(),
                ret_type: mut_rc(UnknownType {}),
                parameters: Vec::new(),
                generic_args: HashMap::new(),
                generic_params_order: method_type
                    .generic_params_order
                    .iter()
                    .map(|p| {
                        let mut renamed = p.clone();
                        renamed.literal = Some(ClassType::rename_generic_param_for_method(
                            p.literal.clone().unwrap(),
                        ));
                        renamed
                    })
                    .collect(),
            };

            let mut method_generics = our_generics.clone();

            for p in method_type.generic_params_order.iter() {
                let mut renamed = p.clone();
                renamed.literal = Some(ClassType::rename_generic_param_for_method(
                    p.literal.clone().unwrap(),
                ));
                new_method_type.generic_args.insert(
                    renamed.clone().literal.unwrap(),
                    mut_rc(GenericType {
                        identifier: renamed.clone(),
                    }),
                );
                method_generics.insert(
                    p.clone().literal.unwrap(),
                    mut_rc(GenericType {
                        identifier: renamed.clone(),
                    }),
                );
            }

            for param in &method_type.parameters {
                let type_ = param.type_.borrow().concrete(&method_generics, cache)?;
                new_method_type.parameters.push(FnParamType {
                    name: param.name.clone(),
                    type_,
                    default_value: param.default_value.clone(),
                    position: param.position.clone(),
                });
            }

            let return_type = method_type
                .ret_type
                .borrow()
                .concrete(&method_generics, cache)?;
            new_method_type.ret_type = return_type;

            res.borrow_mut()
                .methods
                .insert(name.clone(), mut_rc(new_method_type));
        }

        cache.remove(&cache_id);

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
