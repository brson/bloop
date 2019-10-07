use pest;

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
struct Lexer;

use crate::Result;

enum Phase<'a> {
    Pre(usize, Pair<'a, Rule>),
    Post(usize, PostState),
}

enum PostState {
    TokenTree,
    TreeOrThing(TreeOrThing),
    Unimpl,
}

pub fn lex(src: &str) -> Result<TokenTree> {
    debug!("source:\n{}\n", src);

    let pairs = Lexer::parse(Rule::buffer, src)
        .context(format!("parsing source"))?;

    debug!("lexed:");

    let mut pair_stack = vec![];
    let mut last_token_tree = None;
    let mut tree_or_thing_stack = vec![];

    // FIXME: deduplicate this block of code from the one below
    {
        // collect new pairs in forward order
        let mut next_pairs = vec![];
        for next_pair in pairs {
            next_pairs.push(Phase::Pre(0, next_pair));
        }
        // push them on the stack in reverse order so they
        // can be popped in forward order later
        let next_pairs = next_pairs.into_iter().rev();
        pair_stack.extend(next_pairs);
    }

    while let Some(this_phase) = pair_stack.pop() {
        match this_phase {
            Phase::Pre(lvl, this_pair) => {

                {
                    let pad = iter::repeat(' ').take(lvl).collect::<String>();
                    let mut src = this_pair.as_str().to_string();
                    src.truncate(20);
                    let src = src.replace("\n", " ");
                    let src = src.replace("\r\n", " ");
                    debug!("{}{:?}: {}", pad, this_pair.as_rule(), src);
                }

                let next_move = get_post_state(this_pair.as_rule(), this_pair.as_str());

                pair_stack.push(Phase::Post(lvl, next_move));

                {
                    let mut next_pairs = vec![];
                    for next_pair in this_pair.into_inner() {
                        next_pairs.push(Phase::Pre(lvl + 1, next_pair));
                    }
                    let next_pairs = next_pairs.into_iter().rev();
                    pair_stack.extend(next_pairs);
                }
            }
            Phase::Post(_lvl, post_state) => {
                match post_state {
                    PostState::TokenTree => {
                        assert!(last_token_tree.is_none());
                        // pop the "tot" stack completely
                        tree_or_thing_stack.reverse();
                        last_token_tree = Some(TokenTree(tree_or_thing_stack));
                        tree_or_thing_stack = vec![];
                    }
                    PostState::TreeOrThing(tot) => {
                        tree_or_thing_stack.push(tot);
                    }
                    PostState::Unimpl => { }
                }
            }
        }
    }

    assert!(pair_stack.is_empty());
    assert!(tree_or_thing_stack.is_empty());
    assert!(last_token_tree.is_some());

    debug!("... lexed.");

    if let Some(tt) = last_token_tree {
        Ok(tt)
    } else {
        panic!("lexing didn't produce a token tree");
    }
}

fn get_post_state(rule: Rule, s: &str) -> PostState {
    match rule {
        Rule::token_tree => {
            PostState::TokenTree
        }
        Rule::uint => {
            PostState::TreeOrThing(TreeOrThing::Thing(
                Thing::Number(Number::UInt(UInt(s.to_string())))
            ))
        }
        Rule::punct_comma => {
            PostState::TreeOrThing(TreeOrThing::Thing(
                Thing::Punctuation(Punctuation::Comma)
            ))
        }
        _ => {
            PostState::Unimpl
        }
    }
}
