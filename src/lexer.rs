#![allow(unused)]

use std::cell::Cell;
use std::collections::HashMap;

struct TokenTree(Vec<Token>);

struct Token {
    offset: usize,
    ty: TokenType,
}

enum TokenType {
    Ident(String),
    Keyword(String),
    BracedTree(TokenTree),
    BracketedTree(TokenTree),
    AngledTree(TokenTree),
    Toml(String),
    SimpleToken(String),
}

enum Keyword {
    Break,
    Continue,
    If,
    Let,
    Loop,
    Ty,
}

static KEYWORDS: &[&str] = &[
    "break",
    "continue",
    "if",
    "let",
    "loop",
    "ty",
];

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

fn tokenize(src: &str) -> () {
    let i = Cell::new(0);

    
    
}

