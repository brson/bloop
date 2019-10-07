use pest;

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

    {
        let mut next_pair_queue = vec![];
        for pair in pairs {
            next_pair_queue.push(Phase::Pre(0, pair));
        }
        let mut next_pair_stack = next_pair_queue;
        next_pair_stack.reverse();
        pair_stack.extend(next_pair_stack.into_iter());
    }

    let mut last_token_tree = None;
    let mut tree_or_thing_accum = vec![];

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
                    let mut next_pair_queue = vec![];

                    for next_pair in this_pair.into_inner() {
                        next_pair_queue.push(Phase::Pre(lvl + 1, next_pair));
                    }

                    let mut next_pair_stack = next_pair_queue;
                    next_pair_stack.reverse();
                    pair_stack.extend(next_pair_stack.into_iter());
                }
            }
            Phase::Post(_lvl, post_state) => {
                match post_state {
                    PostState::TokenTree => {
                        tree_or_thing_accum.reverse();
                        assert!(last_token_tree.is_none());
                        last_token_tree = Some(TokenTree(tree_or_thing_accum));
                        tree_or_thing_accum = vec![];
                    }
                    PostState::TreeOrThing(tot) => {
                        tree_or_thing_accum.push(tot);
                    }
                    PostState::Unimpl => { }
                }
            }
        }
    }

    assert!(pair_stack.is_empty());

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
