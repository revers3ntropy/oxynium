use crate::target::Target;

pub fn main_fn_id(target: Target) -> String {
    match target {
        Target::X86_64Linux => "main".to_owned(),
        Target::MACOS => "_main".to_owned(),
    }
}
