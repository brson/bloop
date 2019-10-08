use pest;

use crate::big_s::S;
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
    TokenTree, TreeOrThing, Tree, Thing, Ident, Number, Float, Uint, Punctuation,
};

#[derive(Parser)]
#[grammar = "lexer.pest"]
struct PestLexer;

use crate::Result;

pub struct Lexer<'a>(PhantomData<&'a ()>);

impl<'a> Walk for Lexer<'a> {
    type Node = Pair<'a, Rule>;
    type FrameState = TreeOrThing;
    type FrameResult = TreeOrThing;
    
    fn enter_frame(node: Self::Node, mut push_child: impl FnMut(Self::Node)) -> Result<Self::FrameState> {
        let s = node.as_str();
        let state = match node.as_rule() {
            Rule::paren_tree => {
                TreeOrThing::Tree(Tree::Paren, TokenTree(vec![]))
            }
            Rule::ident => {
                TreeOrThing::Thing(Thing::Ident(Ident(S(s))))
            }
            Rule::uint => {
                TreeOrThing::Thing(Thing::Number(Number::Uint(Uint(S(s)))))
            }
            Rule::punct_comma => {
                TreeOrThing::Thing(Thing::Punctuation(Punctuation::Comma))
            }
            r => panic!("unimplemented {:?}", r)
        };

        for pair in node.into_inner() {
            push_child(pair);
        }

        Ok(state)
    }

    fn handle_child_result(mut frm: Self::FrameState, ch: Self::FrameResult) -> Result<Self::FrameState> {
        if let TreeOrThing::Tree(_, ref mut tt) = frm {
            tt.0.push(ch);
        } else {
            panic!("non-tree has children");
        }
        
        Ok(frm)
    }

    fn leave_frame(frm: Self::FrameState) -> Result<Self::FrameResult> {
        Ok(frm)
    }
}
