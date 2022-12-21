use crate::ast::class_declaration::method_id;
use crate::types::function::FnType;
use crate::types::Type;
use crate::util::MutRc;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug)]
pub struct ClassFieldType {
    pub name: String,
    pub type_: MutRc<dyn Type>,
    pub stack_offset: usize,
}
impl ClassFieldType {
    fn str(&self) -> String {
        if self.name == "" {
            self.type_.borrow_mut().str()
        } else {
            format!("{}: {}", self.name, self.type_.borrow_mut().str())
        }
    }
}

#[derive(Clone)]
pub struct ClassType {
    pub name: String,
    pub fields: HashMap<String, ClassFieldType>,
    pub methods: HashMap<String, MutRc<FnType>>,
    pub is_primitive: bool,
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

impl fmt::Debug for ClassType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {{ {} }}",
            self.name,
            self.fields
                .iter()
                .map(|(_, f)| f.str())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl Type for ClassType {
    fn is_ptr(&self) -> bool {
        true
    }

    fn str(&self) -> String {
        self.name.clone()
    }

    fn contains(&self, other: MutRc<dyn Type>) -> bool {
        if other.borrow().is_unknown() {
            return true;
        }
        // compare values of pointers...
        // TODO to this properly
        format!("{:p}", self) == format!("{:p}", other.as_ptr())
    }

    fn as_class(&self) -> Option<ClassType> {
        Some(self.clone())
    }
}
