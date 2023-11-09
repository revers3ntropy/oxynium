#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    X86_64Linux,
    MACOS,
}

impl Target {
    #[cfg(target_os = "macos")]
    fn current() -> Self {
        Target::MACOS
    }

    #[cfg(target_os = "linux")]
    fn current() -> Self {
        Target::X86_64Linux
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Target::X86_64Linux => "x86_64-linux",
            Target::MACOS => "macos",
        }
    }

    pub fn from_str(s: String) -> Self {
        match s.as_str() {
            "macos" => Target::MACOS,
            "x86_64-linux" => Target::X86_64Linux,
            "" => Target::current(),
            _ => {
                eprintln!(
                    "unknown target `{s}`, using `{}`",
                    Target::current().as_str()
                );
                Target::current()
            }
        }
    }
}
