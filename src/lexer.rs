#![allow(unused)]

use std::cell::Cell;
use std::collections::HashMap;

static MATCHED_TOKENS: &[(&str, &str)] = &[
    ("{", "}"),
    ("[", "]"),
    ("<", ">"),
    ("![", "]!"), // TOML delimiters
];

static TOKENS: &[&str] = &[
    ";",
    ".",
    "-",
    "+",
    "/",
    "*",
    "<-",
    "->",
    "=",
    "<=",
    ">=",
    "<.",
    ">.",
    "!",
];

static IDENT_CHARS: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
    'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
    'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    '_'
];

static NON_LEADING_IDENT_CHARS: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'
];

static PRIME_IDENT_TRAILER: &str = "'";
static STRING_DELIM: &str = "\"";
static CHAR_DELIM: &str = "'";
static DOC_COMMENT: &str = "///";
static COMMENT: &str = "//";

enum State {
    Newline,
}

pub fn lex(src: &str) -> () {

    let mut state = State::Newline;

    //let brace_stack = vec![];
    
    for (i, c) in src.char_indices() {
        if c.is_whitespace() {
            continue;
        }
    }
}

