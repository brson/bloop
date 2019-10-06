use pest;

use failure::err_msg;
use failure::ResultExt;
use pest::Parser;
use pest::iterators::{Pairs, Pair};
use std::iter;
use crate::token_tree::{
    TokenTree, TreeOrThing, Tree, Thing, Ident, Number, Float, Int, Punctuation,
};

#[derive(Parser)]
#[grammar = "lexer.pest"]
struct Lexer;

use crate::Result;

pub fn lex(src: &str) -> Result<TokenTree> {
    debug!("source:\n{}\n", src);

    let pairs = Lexer::parse(Rule::module, src)
        .context(format!("parsing source"))?;

    debug!("lexed:");

    enum Phase<'a> {
        Pre(usize, Pair<'a, Rule>),
        Post(usize),
    };

    let mut pair_stack = vec![];

    for pair in pairs {
        pair_stack.push(Phase::Pre(0, pair));
    }

    while let Some(this_phase) = pair_stack.pop() {
        match this_phase {
            Phase::Pre(lvl, this_pair) => {

                let pad = iter::repeat(' ').take(lvl).collect::<String>();
                let mut src = this_pair.as_str().to_string();
                src.truncate(20);
                let src = src.replace("\n", " ");
                let src = src.replace("\r\n", " ");
                debug!("{}{:?}: {}", pad, this_pair.as_rule(), src);

                match this_pair.as_rule() {
                    Rule::module => {
                    }
                    _ => { }
                }

                let mut next_pair_queue = vec![];
                next_pair_queue.push((Phase::Post(lvl)));

                for next_pair in this_pair.into_inner() {
                    next_pair_queue.push((Phase::Pre(lvl + 1, next_pair)));
                }

                let mut next_pair_stack = next_pair_queue;
                next_pair_stack.reverse();
                pair_stack.append(&mut next_pair_stack);
            }
            Phase::Post(..) => {
            }
        }
    }

    assert!(pair_stack.is_empty());

    debug!("... lexed.");

    panic!();
}

