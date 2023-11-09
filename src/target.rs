#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    X86_64Linux,
    MACOS,
}

impl Target {
    pub fn from_str(s: String) -> Self {
        match s.as_str() {
            "macos" => Target::MACOS,
            "" | "x86" | "linux" | "x86_64" => Target::X86_64Linux,
            _ => {
                eprintln!("Unknown target: {}", s);
                Target::X86_64Linux
            }
        }
    }
}
