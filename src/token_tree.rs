// NB: Keep the ordering here the same as in lexer.pest

#[derive(Debug)]
pub struct TokenTree(pub Vec<TreeOrThing>);

#[derive(Debug)]
pub enum TreeOrThing {
    Tree(Tree, TokenTree),
    Thing(Thing),
}

#[derive(Debug)]
pub enum Tree {
    Paren,
    Brace,
    Square,
    Angle,
}

#[derive(Debug)]
pub enum Thing {
    Ident(Ident),
    Number(Number),
    Punctuation(Punctuation),
}

#[derive(Debug)]
pub struct Ident(pub String);

#[derive(Debug)]
pub enum Number {
    Float(Float),
    Uint(Uint),
}

#[derive(Debug)]
pub struct Float(pub String);
#[derive(Debug)]
pub struct Uint(pub String);

#[derive(Debug)]
pub enum Punctuation {
    RightArrow,
    LeftArrow,
    FatRightArrow,
    FatLeftArrow,

    DotDotEquals,
    DotDotDot,
    DoteDot,
    Dot,

    ColonEquals,
    ColonColon,
    Colon,

    Semicolon,
    Comma,

    BarBar,
    Bar,
    AndAnd,
    And,

    Plus,
    Dash,
    Slash,
    Star,

    PlusEquals,
    DashEquals,
    SlashEquals,
    StarEquals,

    EqualsEquals,
    Equals,

    Backtick,
    At,
    Spider,
    Dollar,
    Percent,
    UpArrow,
    Twiddle,
    Bang,
    What,
    Backslash,
}
