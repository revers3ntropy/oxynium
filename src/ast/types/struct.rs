use crate::ast::types::Type;
use std::fmt;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct StructFieldType {
    pub name: String,
    pub type_: Rc<dyn Type>,
}
impl StructFieldType {
    fn str(&self) -> String {
        if self.name == "" {
            self.type_.str()
        } else {
            format!("{}: {}", self.name, self.type_.str())
        }
    }
}

#[derive(Clone)]
pub struct StructType {
    pub name: String,
    pub fields: Vec<StructFieldType>,
}

impl StructType {
    pub fn field_type(&self, field: &str) -> Option<Rc<dyn Type>> {
        self.fields
            .iter()
            .find(|f| f.name == field)
            .map(|f| f.type_.clone())
    }

    pub fn field_offset(&self, field: String) -> usize {
        self.fields.iter().position(|f| f.name == field).unwrap() * 8
    }
}

impl fmt::Debug for StructType {
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

impl Type for StructType {
    fn is_ptr(&self) -> bool {
        true
    }
    fn str(&self) -> String {
        self.name.clone()
    }

    fn contains(&self, t: Rc<dyn Type>) -> bool {
        // compare values of pointers...
        // TODO to this properly
        format!("{:p}", self) == format!("{:p}", t.as_ref())
    }

    fn as_struct(&self) -> Option<StructType> {
        Some(self.clone())
    }
}
