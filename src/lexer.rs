use pest;

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
struct Lexer;

use crate::Result;

enum Phase<'a> {
    Pre(u32, Pair<'a, Rule>),
    Post(u32, Rule, PostState),
}

#[derive(Debug)]
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
                print_phase_debug(PhaseName::Pre, lvl, this_pair.as_rule(), this_pair.as_str());

                let next_move = get_post_state(this_pair.as_rule(), this_pair.as_str());

                pair_stack.push(Phase::Post(lvl, this_pair.as_rule(), next_move));

                let next_lvl = lvl.checked_add(1).expect("level exceeds u32 capacity");
                push_next_pairs(&mut pair_stack, this_pair.into_inner(), next_lvl);
            }
            Phase::Post(lvl, rule, post_state) => {
                print_phase_debug(PhaseName::Post, lvl, rule, &format!("{:?}", post_state));

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
        next_pair_stack.push(Phase::Pre(lvl, next_pair));
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

enum PhaseName { Pre, Post }

fn print_phase_debug(phase: PhaseName, lvl: u32, rule: Rule, s: &str) {
    let sign = match phase {
        PhaseName::Pre => "^",
        PhaseName::Post => "v",
    };
    let spaces = usize::try_from(lvl).expect("lvl does not fit in usize");
    let pad = iter::repeat(' ').take(spaces).collect::<String>();
    let s = &s[..cmp::min(20, s.len())];
    let s = s.to_string();
    let s = s.replace("\n", " ");
    let s = s.replace("\r\n", " ");
    debug!("{} {}{:?}: {}", sign, pad, rule, s);
}
