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

pub fn lex(src: &str) -> Result<TokenTree> {
    debug!("source:\n{}\n", src);

    let pairs = Lexer::parse(Rule::file, src)
        .context(format!("parsing source"))?;

    debug!("lexed:");

    enum Phase<'a> {
        Pre(usize, Pair<'a, Rule>),
        Post(usize, PostState),
    };

    enum PostState {
        TokenTree,
        TreeOrThing(TreeOrThing),
        Unimpl,
    }

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

                let next_move;

                match this_pair.as_rule() {
                    Rule::token_tree => {
                        next_move = PostState::TokenTree;
                    }
                    Rule::uint_u32_base10 => {
                        let s = this_pair.as_str().to_string();
                        next_move = PostState::TreeOrThing(
                            TreeOrThing::Thing(
                                Thing::Number(Number::UInt(UInt(s)))
                            )
                        );
                    }
                    Rule::punct_comma => {
                        next_move = PostState::TreeOrThing(
                            TreeOrThing::Thing(
                                Thing::Punctuation(Punctuation::Comma)
                            )
                        );
                    }
                    _ => {
                        next_move = PostState::Unimpl;
                    }
                }

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
                    }
                    PostState::TreeOrThing(tot) => {
                    }
                    PostState::Unimpl => { }
                }
            }
        }
    }

    assert!(pair_stack.is_empty());

    debug!("... lexed.");

    panic!();
}
