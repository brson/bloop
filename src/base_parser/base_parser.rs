#![allow(unused)]

use b_base_ast::{BaseAst, Module};
use b_base_parser_traits::BaseParse;
use b_error::{BResult, BError};
use b_token_tree::TokenTree;
use b_tree_walker::Walk;
use b_base_ast::{
    Declaration,
};
use b_base_parser_lalrpop::parse_module;

pub struct BaseParser;

impl BaseParse for BaseParser {
    fn parse(&self, tt: &TokenTree) -> BResult<BaseAst> {
        // FIXME bad clone
        let node = Node(CurrentTarget::Module, tt.clone());
        let mut parsed = Node::walk(Some(node))?;

        let ast = parsed.pop().expect("parse results");
        assert!(parsed.is_empty());

        match ast.0 {
            AstNode::Module(m) => {
                return Ok(BaseAst(m));
            }
            _ => {
                return Err(BError::new("wrong parse result"));
            }
        }

        Ok(BaseAst(Module { decls: vec![] }))
    }
}

pub enum AstNode {
    Module(Module),
    Declaration(Declaration),
}

struct Node(CurrentTarget, TokenTree);

enum CurrentTarget {
    Module,
    ArgList,
    Body,
}

struct FrameState;

struct FrameResult(AstNode);

// This is going to traverse the token tree, parsing each flat vector of
// ThingOrTree's into a vector of AST items. It decends in parallel into
// sub-trees.

use b_base_partial_ast::{PartialDeclaration, PartialFunction};

impl Walk for Node {
    type Node = Node;
    type FrameState = FrameState;
    type FrameResult = FrameResult;

    // FIXME bad clones
    fn enter_frame(node: Self::Node, mut push_child: impl FnMut(Self::Node)) -> BResult<Option<Self::FrameState>> {
        match node.0 {
            CurrentTarget::Module => {
                let ast = parse_module(&node.1)?;

                for decl in &ast.decls {
                    match decl {
                        PartialDeclaration::Function(
                            PartialFunction {
                                ref args,
                                ref body,
                                ..
                            }
                        ) => {
                            push_child(Node(CurrentTarget::ArgList, (args.0).0.clone()));
                        }
                    }
                }
            },
            CurrentTarget::ArgList => {
                panic!()
            }
            CurrentTarget::Body => {
                panic!()
            }
        }

        Ok(Some(FrameState))
    }

    fn handle_child_result(frm: Self::FrameState, ch: Self::FrameResult) -> BResult<Self::FrameState> {
        panic!()
    }

    fn leave_frame(frm: Self::FrameState) -> BResult<Self::FrameResult> {
        panic!()
    }

}

