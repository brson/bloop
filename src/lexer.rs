use pest;

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
struct Lexer;

use crate::Result;

enum Phase<'a> {
    Pre(u32, Pair<'a, Rule>),
    Post(u32, PostState),
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

    push_next_pairs(&mut pair_stack, pairs, 0);

    while let Some(this_phase) = pair_stack.pop() {
        match this_phase {
            Phase::Pre(lvl, this_pair) => {
                {
                    let spaces = usize::try_from(lvl).expect("lvl does not fit in usize");
                    let pad = iter::repeat(' ').take(spaces).collect::<String>();
                    let mut src = this_pair.as_str().to_string();
                    src.truncate(20);
                    let src = src.replace("\n", " ");
                    let src = src.replace("\r\n", " ");
                    debug!("{}{:?}: {}", pad, this_pair.as_rule(), src);
                }

                let next_move = get_post_state(this_pair.as_rule(), this_pair.as_str());

                pair_stack.push(Phase::Post(lvl, next_move));

                let next_lvl = lvl.checked_add(1).expect("level exceeds u32 capacity");
                push_next_pairs(&mut pair_stack, this_pair.into_inner(), next_lvl);
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

fn push_next_pairs<'a>(pair_stack: &mut Vec<Phase<'a>>,
                       mut next_pairs: Pairs<'a, Rule>,
                       lvl: u32) {
    // collect new pairs in forward order
    let mut next_pair_stack = vec![];
    for next_pair in next_pairs {
        next_pair_stack.push(Phase::Pre(0, next_pair));
    }
    // push them on the stack in reverse order so they
    // can be popped in forward order later
    let next_pair_stack = next_pair_stack.into_iter().rev();
    pair_stack.extend(next_pair_stack);
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
