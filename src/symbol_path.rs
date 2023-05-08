#[derive(Debug, Clone)]
pub struct SymbolPath {
    parts: Vec<String>,
    is_local: bool,
}

impl SymbolPath {
    pub fn new() -> Self {
        Self { parts: Vec::new() }
    }

    pub fn push(&mut self, part: String) {
        self.parts.push(part);
    }

    pub fn part_of(&mut self, other: &Self) {
        let mut self_parts = self.parts.clone();
        self.parts = other.parts.clone();
        self.parts.append(&mut self_parts);
    }

    pub fn to_string(&self) -> String {
        self.parts.join(".")
    }
}
