#![allow(unused)]

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
    use b_token_tree::ThingOrTree;
    use std::slice::Iter;

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
}

#[cfg(tests)]
mod tests {
    #[test]
    fn test() {
        let src = include_str!("../../examples/main.bloop-bl");
    }
}
