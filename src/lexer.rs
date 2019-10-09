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

pub fn lex(src: &str) -> Result<TokenTree> {
    let pairs = PestLexer::parse(Rule::buffer, src)
        .context(format!("parsing source"))?;

    Ok(TokenTree(Lexer::walk(pairs)?))
}

pub struct Lexer<'a>(PhantomData<&'a ()>);

impl<'a> Walk for Lexer<'a> {
    type Node = Pair<'a, Rule>;
    type FrameState = TreeOrThing;
    type FrameResult = TreeOrThing;
    
    fn enter_frame(node: Self::Node, mut push_child: impl FnMut(Self::Node)) -> Result<Option<Self::FrameState>> {
        let state = pair_to_tree_or_thing(&node);

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

fn pair_to_tree_or_thing(p: &Pair<Rule>) -> Option<TreeOrThing> {
    let s = p.as_str();
    let tot = match p.as_rule() {
        Rule::paren_tree => {
            Some(TreeOrThing::Tree(Tree::Paren, TokenTree(vec![])))
        }
        Rule::brace_tree => {
            Some(TreeOrThing::Tree(Tree::Brace, TokenTree(vec![])))
        }
        Rule::square_tree => {
            Some(TreeOrThing::Tree(Tree::Square, TokenTree(vec![])))
        }
        Rule::angle_tree => {
            Some(TreeOrThing::Tree(Tree::Angle, TokenTree(vec![])))
        }
        Rule::ident => {
            Some(TreeOrThing::Thing(Thing::Ident(Ident(S(s)))))
        }
        Rule::uint => {
            Some(TreeOrThing::Thing(Thing::Number(Number::Uint(Uint(S(s))))))
        }
        Rule::punct_comma => {
            Some(TreeOrThing::Thing(Thing::Punctuation(Punctuation::Comma)))
        }
        Rule::EOI => {
            None
        }
        r => panic!("unimplemented {:?}", r)
    };

    tot
}
