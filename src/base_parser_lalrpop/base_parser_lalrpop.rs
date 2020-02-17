#![allow(unused)]

use b_token_tree::TokenTree;
use b_error::{BError, BResult, StdResultExt};
use b_base_partial_ast::{PartialModule, PartialArgList, PartialBody};
use crate::lexer::{Lexer, Spanned, Token};
use crate::parsers::module::ModuleParser;
use crate::parsers::arg_list::ArgListParser;
use crate::parsers::body::BodyParser;
use lalrpop_util::ParseError;

type BParseError = ParseError<usize, Token, BError>;

fn lalrpop_err(e: BParseError) -> BError {
    // FIXME encapsulate error better
    BError::new(format!("parse error: {:?}", e))
}

pub fn parse_module(tt: &TokenTree) -> BResult<PartialModule> {
    let lexer = Lexer::new(&tt.0);
    let parser = ModuleParser::new();
    let ast = parser.parse(lexer);
    let ast = ast.map_err(lalrpop_err)?;
    Ok(ast)
}

pub fn parse_arg_list(tt: &TokenTree) -> BResult<PartialArgList> {
    let lexer = Lexer::new(&tt.0);
    let parser = ArgListParser::new();
    let ast = parser.parse(lexer);
    let ast = ast.map_err(lalrpop_err)?;
    Ok(ast)
}

pub fn parse_body(tt: &TokenTree) -> BResult<PartialBody> {
    let lexer = Lexer::new(&tt.0);
    let parser = BodyParser::new();
    let ast = parser.parse(lexer);
    let ast = ast.map_err(lalrpop_err)?;
    Ok(ast)
}

mod parsers {
    pub mod module;
    pub mod arg_list;
    pub mod body;
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
        TokenTree, Int32, Number,
    };

    #[derive(Debug, Clone)]
    pub enum Token {
        IdentLet,
        IdentConst,
        IdentFn,
        IdentInt32,
        IdentReturn,
        Punctuation(Punctuation),
        Ident(Ident),
        ParenTree(TokenTree),
        BraceTree(TokenTree),
        Int32(Int32),
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
                if s == "let" => Token::IdentLet,
            ToT::Thing(Thing::Ident(Ident(s)))
                if s == "const" => Token::IdentConst,
            ToT::Thing(Thing::Ident(Ident(s)))
                if s == "fn" => Token::IdentFn,
            ToT::Thing(Thing::Ident(Ident(s)))
                if s == "Int32" => Token::IdentInt32,
            ToT::Thing(Thing::Ident(Ident(s)))
                if s == "return" => Token::IdentReturn,
            ToT::Thing(Thing::Ident(i))
                => Token::Ident(i.clone()),
            ToT::Thing(Thing::Punctuation(p))
                => Token::Punctuation(*p),
            ToT::Thing(Thing::Number(Number::Int32(u)))
                => Token::Int32(u.clone()),
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

        pub fn lit_string(self) -> String {
            match self {
                Token::Int32(Int32(s)) => s,
                _ => panic!("not a literal"),
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
