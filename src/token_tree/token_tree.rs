// NB: Keep the ordering here the same as in lexer.pest

// FIXME expensive clones

#[derive(Debug)]
#[derive(Clone)]
pub struct TokenTree(pub Vec<ThingOrTree>);

#[derive(Debug)]
#[derive(Clone)]
pub enum ThingOrTree {
    Thing(Thing),
    Tree(Tree),
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Tree(pub TreeType, pub TokenTree);

#[derive(Debug)]
#[derive(Clone)]
pub enum TreeType {
    Paren,
    Brace,
    Square,
    Angle,
}

#[derive(Debug)]
#[derive(Clone)]
pub enum Thing {
    Ident(Ident),
    Number(Number),
    Punctuation(Punctuation),
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Ident(pub String);

#[derive(Debug)]
#[derive(Clone)]
pub enum Number {
    Uint(Uint),
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Uint(pub String);

#[derive(Debug)]
#[derive(Copy, Clone)]
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

    PlusPlus,
    DashDash,
    SlashSlash,
    StarStar,

    PlusEquals,
    DashEquals,
    SlashEquals,
    StarEquals,

    Plus,
    Dash,
    Slash,
    Star,

    EqualsEquals,
    Equals,

    Quote,
    DQuote,
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
