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
pub struct PartialModule {
    pub decls: Vec<PartialDeclaration>,
}

#[derive(Debug)]
pub enum PartialDeclaration {
    Function(PartialFunction),
}

#[derive(Debug)]
pub struct PartialFunction {
    pub name: Ident,
    pub args: ArgListTree,
    pub ret: RetDecl,
    pub body: BodyTree,
}

pub use b_base_ast::Ident;

#[derive(Debug)]
pub struct ArgListTree(pub ParenTree);

#[derive(Debug)]
pub struct PartialArgList {
    args: Vec<Argument>,
}

pub use b_base_ast::RetDecl;

pub use b_base_ast::Argument;

#[derive(Debug)]
pub struct BodyTree(pub BraceTree);

#[derive(Debug)]
pub struct PartialBody;

pub use b_base_ast::Const;

pub use b_base_ast::Literal;

pub use b_base_ast::Type;
