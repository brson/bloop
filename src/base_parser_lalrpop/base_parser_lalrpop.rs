#![allow(unused)]

use b_token_tree::TokenTree;
use b_error::{BError, BResult, StdResultExt};
use b_base_ast::Module;
use crate::lexer::{Lexer, Spanned};
use crate::parsers::module::ModuleParser;

pub fn parse_module(tt: &TokenTree) -> BResult<Module> {
    let lexer = Lexer::new(&tt.0);
    let parser = ModuleParser::new();
    let ast = parser.parse(lexer);
    let ast = match ast {
        Ok(ast) => ast,
        Err(e) => {
            // FIXME encapsulate error better
            return Err(BError::new(format!("parse error: {:?}", e)));
        }
    };
    // FIXME put this is the parser
    let ast = Module { decls: ast };
    Ok(ast)
}

mod parsers {
    pub mod module;
}

mod ast {
    pub use b_base_ast::*;
}

mod lexer {
    use std::str::CharIndices;
    use std::result::Result;
    use b_error::{BError, BResult};
    use std::slice::Iter;

    pub use b_token_tree::*;

    pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

    pub struct Lexer<'a> {
        tokens: Iter<'a, ThingOrTree>,
    }

    impl<'a> Lexer<'a> {
        pub fn new(input: &'a [ThingOrTree]) -> Self {
            Lexer {
                tokens: input.iter(),
            }
        }
    }

    impl<'a> Iterator for Lexer<'a> {
        type Item = Spanned<ThingOrTree, usize, BError>;

        fn next(&mut self) -> Option<Self::Item> {
            if let Some(next) = self.tokens.next() {
                // FIXME clone
                let next = (0, next.clone(), 0);
                Some(Ok(next))
            } else {
                None
            }
        }
    }
}

#[cfg(tests)]
mod tests {
    #[test]
    fn test() {
        let src = include_str!("../../examples/main.bloop-bl");
    }
}
