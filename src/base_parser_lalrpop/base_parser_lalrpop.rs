#![allow(unused)]

use b_token_tree::TokenTree;
use b_error::{BError, BResult, StdResultExt};
use b_base_partial_ast::PartialModule;
use crate::lexer::{Lexer, Spanned};
use crate::parsers::module::ModuleParser;

pub fn parse_module(tt: &TokenTree) -> BResult<PartialModule> {
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
    Ok(ast)
}

mod parsers {
    pub mod module;
}

mod ast {
    pub use b_base_partial_ast::*;
}

mod lexer {
    use std::str::CharIndices;
    use std::result::Result;
    use b_error::{BError, BResult};
    use std::slice::Iter;
    use b_token_tree::ThingOrTree;

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
        type Item = Spanned<Token, usize, BError>;

        fn next(&mut self) -> Option<Self::Item> {
            if let Some(next) = self.tokens.next() {
                let token = thing_or_tree_to_token(&next);
                let next = (0, token, 0);
                Some(Ok(next))
            } else {
                None
            }
        }
    }

    pub use b_token_tree::{
        Punctuation, Ident,
        TokenTree,
    };

    #[derive(Debug, Clone)]
    pub enum Token {
        IdentFn,
        IdentI32,
        Punctuation(Punctuation),
        Ident(Ident),
        ParenTree(TokenTree),
        BraceTree(TokenTree),
        Unimplemented,
    }

    fn thing_or_tree_to_token(tot: &ThingOrTree) -> Token {
        use b_token_tree::ThingOrTree as ToT;
        use b_token_tree::{
            Tree, Thing, TreeType
        };

        // FIXME expensive clones
        match tot {
            ToT::Thing(Thing::Ident(Ident(s)))
                if s == "fn" => Token::IdentFn,
            ToT::Thing(Thing::Ident(Ident(s)))
                if s == "I32" => Token::IdentI32,
            ToT::Thing(Thing::Ident(i))
                => Token::Ident(i.clone()),
            ToT::Thing(Thing::Punctuation(p))
                => Token::Punctuation(*p),
            ToT::Tree(Tree(TreeType::Paren, t))
                => Token::ParenTree(t.clone()),
            ToT::Tree(Tree(TreeType::Brace, t))
                => Token::BraceTree(t.clone()),
            _ => panic!("unimplemented tt conversion: {:?}", tot)
        }
    }

    impl Token {
        pub fn ident_string(self) -> String {
            match self {
                Token::Ident(Ident(s)) => s,
                _ => panic!("not an ident"),
            }
        }

        pub fn tree(self) -> TokenTree {
            match self {
                Token::ParenTree(t) => t,
                Token::BraceTree(t) => t,
                _ => panic!("not a tree"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use b_lexer::Lexer;
    use b_lexer_traits::Lex;
    use crate::parse_module;

    #[test]
    fn test() {
        let src = include_str!("../../examples/main.bloop-bl");
        let lexer = Lexer;
        let tt = lexer.lex(src).unwrap();
        let _ast = parse_module(&tt).unwrap();
    }
}
