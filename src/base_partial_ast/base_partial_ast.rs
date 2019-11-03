#![allow(unused)]

// This file mirrors base_ast.rs except that any subtrees are substituted with
// one of the `tree` token tree types.

mod trees {
    use b_token_tree::TokenTree;

    #[derive(Debug)]
    pub struct ParenTree(pub TokenTree);

    #[derive(Debug)]
    pub struct BraceTree(pub TokenTree);

    #[derive(Debug)]
    pub struct SquareTree(pub TokenTree);

    #[derive(Debug)]
    pub struct AngleTree(pub TokenTree);
}

pub use trees::*;

#[derive(Debug)]
pub struct Module {
    pub decls: Vec<Declaration>,
}

#[derive(Debug)]
pub enum Declaration {
    Function(Function),
}

#[derive(Debug)]
pub struct Function {
    pub name: Ident,
    pub args: ArgList,
    pub ret: RetDecl,
    pub body: Body,
}

pub use b_base_ast::Ident;

#[derive(Debug)]
pub struct ArgList(pub ParenTree);

pub use b_base_ast::RetDecl;

pub use b_base_ast::Argument;

#[derive(Debug)]
pub struct Body(pub BraceTree);

pub use b_base_ast::Const;

pub use b_base_ast::Literal;

pub use b_base_ast::Type;
