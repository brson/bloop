use pest;

use std::marker::PhantomData;
use crate::tree_walker::Walk;
use std::cmp;
use std::convert::TryFrom;
use std::collections::VecDeque;
use failure::err_msg;
use failure::ResultExt;
use pest::Parser;
use pest::iterators::{Pairs, Pair};
use std::iter;
use crate::token_tree::{
    TokenTree, TreeOrThing, Tree, Thing, Ident, Number, Float, UInt, Punctuation,
};

#[derive(Parser)]
#[grammar = "lexer.pest"]
struct PestLexer;

use crate::Result;

pub struct Lexer<'a>(PhantomData<&'a ()>);

pub struct Node<'a>(Pair<'a, Rule>);
pub struct FrameState;
pub struct FrameResult(TreeOrThing);

impl<'a> Walk for Lexer<'a> {
    type Node = Node<'a>;
    type FrameState = FrameState;
    type FrameResult = FrameResult;
    
    fn enter_frame(node: Self::Node) -> Result<Self::FrameState> {
        panic!()
    }

    fn visit_child(frm: Self::FrameState, ch: Self::FrameResult) -> Result<Self::FrameState> {
        panic!()
    }

    fn leave_frame(frm: Self::FrameState) -> Result<Self::FrameResult> {
        panic!()
    }
}
