#![allow(unused)]

use b_base_ast::{BaseAst, Module};
use b_base_parser_traits::BaseParse;
use b_error::{BResult, BError};
use b_token_tree::TokenTree;
use b_tree_walker::Walk;
use b_base_ast::{
    Declaration,
};

pub struct BaseParser;

impl BaseParse for BaseParser {
    fn parse(&self, tt: &TokenTree) -> BResult<BaseAst> {
        let node = Node(tt);
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

struct Node<'a>(&'a TokenTree);

struct FrameState;

struct FrameResult(AstNode);

impl<'a> Walk for Node<'a> {
    type Node = Node<'a>;
    type FrameState = FrameState;
    type FrameResult = FrameResult;

    fn enter_frame(node: Self::Node, push_child: impl FnMut(Self::Node)) -> BResult<Option<Self::FrameState>> {
        panic!()
    }

    fn handle_child_result(frm: Self::FrameState, ch: Self::FrameResult) -> BResult<Self::FrameState> {
        panic!()
    }

    fn leave_frame(frm: Self::FrameState) -> BResult<Self::FrameResult> {
        panic!()
    }

}

