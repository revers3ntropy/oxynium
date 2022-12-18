use crate::types::function::FnType;
use crate::types::Type;
use crate::util::MutRc;
use std::fmt;
use std::ops::Deref;
use crate::ast::class_declaration::method_id;

#[derive(Clone, Debug)]
pub struct ClassFieldType {
    pub name: String,
    pub type_: MutRc<dyn Type>,
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
    pub fields: Vec<ClassFieldType>,
    pub methods: Vec<MutRc<FnType>>,
}

impl ClassType {
    pub fn field_type(&self, field: &str) -> Option<MutRc<dyn Type>> {
        self.fields
            .iter()
            .find(|f| f.name == field)
            .map(|f| f.type_.clone())
    }

    pub fn method_type(&self, method: &str) -> Option<MutRc<FnType>> {
        self.methods
            .iter()
            .find(|f| f.borrow().name == method_id(
                self.name.clone(), method.to_string()))
            .map(|f| f.clone())
    }

    pub fn field_offset(&self, field: String) -> usize {
        self.fields.iter().position(|f| f.name == field).unwrap() * 8
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
                .map(|f| f.str())
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

    fn contains(&self, t: MutRc<dyn Type>) -> bool {
        // compare values of pointers...
        // TODO to this properly
        format!("{:p}", self) == format!("{:p}", t.borrow().deref())
    }

    fn as_class(&self) -> Option<ClassType> {
        Some(self.clone())
    }
}
