#![allow(unused)]

// This file mirrors base_ast.rs except that any subtrees are substituted with
// one of the `tree` token tree types.

mod trees {
    use b_token_tree::TokenTree;

    pub struct ParenTree(pub TokenTree);
    pub struct BraceTree(pub TokenTree);
    pub struct SquareTree(pub TokenTree);
    pub struct AngleTree(pub TokenTree);
}

use trees::*;

pub struct Module {
    pub decls: Vec<Declaration>,
}

pub enum Declaration {
    Function(Function),
}

pub struct Function {
    pub name: Ident,
    pub args: ArgList,
    pub ret: RetDecl,
    pub body: Body,
}

pub use b_base_ast::Ident;

pub struct ArgList(ParenTree);

pub use b_base_ast::RetDecl;

pub use b_base_ast::Argument;

pub struct Body(BraceTree);

pub use b_base_ast::Const;

pub use b_base_ast::Literal;

pub use b_base_ast::Type;
