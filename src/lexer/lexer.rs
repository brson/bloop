#[macro_use]
extern crate pest_derive;

use b_deps::anyhow::{Result, Context};
use b_lexer_traits::Lex;
use b_token_tree::{
    TokenTree, ThingOrTree, Tree, TreeType, Thing, Ident, Number, Int32, Punctuation,
};
use b_tree_walker::Walk;
use pest::Parser;
use pest::iterators::Pair;
use pest;
use std::marker::PhantomData;

pub struct Lexer;

impl Lex for Lexer {
    fn lex(&self, src: &str) -> Result<TokenTree> {
        let pairs = PestLexer::parse(Rule::buffer, src)
            .context(format!("parsing source"))?;

        Ok(TokenTree(LocalLexer::walk_maybe_part_par(pairs)?))
    }
}

// FIXME: lexer.pest is located in src/ because pest expects it to be there. I
// would rather it not.
//
// re https://github.com/pest-parser/pest/issues/325
#[derive(Parser)]
#[grammar = "lexer.pest"]
struct PestLexer;

// FIXME PhantomData is a limitation of not gaving generic associated types
pub struct LocalLexer<'a>(PhantomData<&'a ()>);

impl<'a> Walk for LocalLexer<'a> {
    type Node = Pair<'a, Rule>;
    type FrameState = ThingOrTree;
    type FrameResult = ThingOrTree;
    
    fn enter_frame(node: Self::Node, mut push_child: impl FnMut(Self::Node)) -> Result<Option<Self::FrameState>> {
        let state = pair_to_tree_or_thing(&node);

        for pair in node.into_inner() {
            push_child(pair);
        }

        Ok(state)
    }

    fn handle_child_result(mut frm: Self::FrameState, ch: Self::FrameResult) -> Result<Self::FrameState> {
        if let ThingOrTree::Tree(Tree(_, ref mut tt)) = frm {
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

fn pair_to_tree_or_thing(p: &Pair<Rule>) -> Option<ThingOrTree> {
    let s = p.as_str();
    let tot = match p.as_rule() {
        Rule::paren_tree => {
            Some(ThingOrTree::Tree(Tree(TreeType::Paren, TokenTree(vec![]))))
        }
        Rule::brace_tree => {
            Some(ThingOrTree::Tree(Tree(TreeType::Brace, TokenTree(vec![]))))
        }
        Rule::square_tree => {
            Some(ThingOrTree::Tree(Tree(TreeType::Square, TokenTree(vec![]))))
        }
        Rule::angle_tree => {
            Some(ThingOrTree::Tree(Tree(TreeType::Angle, TokenTree(vec![]))))
        }
        Rule::ident => {
            Some(ThingOrTree::Thing(Thing::Ident(Ident(s.to_string()))))
        }
        Rule::int32 => {
            Some(ThingOrTree::Thing(Thing::Number(Number::Int32(Int32(s.to_string())))))
        }
        Rule::punct_right_arrow => {
            Some(ThingOrTree::Thing(Thing::Punctuation(Punctuation::RightArrow)))
        }
        Rule::punct_colon => {
            Some(ThingOrTree::Thing(Thing::Punctuation(Punctuation::Colon)))
        }
        Rule::punct_semicolon => {
            Some(ThingOrTree::Thing(Thing::Punctuation(Punctuation::Semicolon)))
        }
        Rule::punct_comma => {
            Some(ThingOrTree::Thing(Thing::Punctuation(Punctuation::Comma)))
        }
        Rule::punct_equals => {
            Some(ThingOrTree::Thing(Thing::Punctuation(Punctuation::Equals)))
        }
        Rule::EOI => {
            None
        }
        r => panic!("unimplemented {:?}", r)
    };

    tot
}
