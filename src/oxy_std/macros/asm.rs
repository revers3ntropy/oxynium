use crate::ast::raw_asm::RawAsmNode;
use crate::ast::symbol_access::SymbolAccess;
use crate::ast::AstNode;
use crate::context::Context;
use crate::error::{type_error, Error};
use crate::oxy_std::macros::Macro;
use crate::parse::token::{Token, TokenType};
use crate::position::Interval;
use crate::util::new_mut_rc;
use crate::util::MutRc;

pub struct AsmMacro {
    pub position: Interval,
    pub args: Vec<MutRc<dyn AstNode>>,
}

impl Macro for AsmMacro {
    fn resolve(
        &self,
        _ctx: MutRc<dyn Context>,
    ) -> Result<MutRc<dyn AstNode>, Error> {
        let mut args = self.args.clone();

        if args.len() != 1 {
            return Err(type_error(format!(
                "macro `asm` takes exactly 1 argument"
            ))
            .set_interval(self.position.clone()));
        }
        let arg = args.remove(0);
        if let Some(as_str_node) =
            arg.borrow().as_str_node()
        {
            return Ok(new_mut_rc(RawAsmNode {
                asm: as_str_node.value.clone(),
                return_type: new_mut_rc(SymbolAccess {
                    identifier: Token::new(
                        TokenType::Identifier,
                        Some("Void".to_string()),
                        self.position.0.clone(),
                        self.position.1.clone(),
                    ),
                }),
            }));
        }
        return Err(type_error(format!(
            "Argument to macro `asm` must be a string literal"
        )).set_interval(self.position.clone()));
    }
}
