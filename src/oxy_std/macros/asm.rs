use crate::ast::raw_asm::RawAsmNode;
use crate::ast::symbol_access::SymbolAccess;
use crate::ast::Node;
use crate::context::Context;
use crate::error::{type_error, Error};
use crate::oxy_std::macros::Macro;
use crate::parse::token::{Token, TokenType};
use crate::position::Interval;
use crate::util::new_mut_rc;
use crate::util::MutRc;

pub struct AsmMacro {
    pub position: Interval,
    pub args: Vec<MutRc<dyn Node>>,
}

impl Macro for AsmMacro {
    fn resolve(
        &self,
        _ctx: MutRc<Context>,
    ) -> Result<MutRc<dyn Node>, Error> {
        let mut args = self.args.clone();

        if args.len() == 1 {
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
            return Ok(arg);
        }

        if args.len() != 2 {
            return Err(type_error(format!(
                "macro `asm` takes exactly 1-2 arguments"
            ))
            .set_interval(self.position.clone()));
        }

        if args[0].borrow().as_symbol_access().is_none() {
            return Err(type_error(format!(
                "First argument to macro `asm` must be a type"
            )).set_interval(self.position.clone()));
        }

        let str_node = args[1].borrow().as_str_node();

        if str_node.is_none() {
            return  Err(type_error(format!(
                "Second argument to macro `asm` must be a string literal"
            )).set_interval(self.position.clone()));
        }

        let return_type = new_mut_rc(
            args.remove(0)
                .borrow()
                .as_symbol_access()
                .unwrap(),
        );

        let str_node = str_node.unwrap();
        return Ok(new_mut_rc(RawAsmNode {
            asm: str_node.value.clone(),
            return_type,
        }));
    }
}
