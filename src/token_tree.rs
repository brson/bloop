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
    TripleDot,
    DoubleDot,
    Dot,
    Bang,
    DoubleColon,
    Colon,
    SemiColon,
    Bar,
    Plus,
    Minus,
    Slash,
    Star,
    Spider,
    Twiddle,
    Tick,
    Dollar,
    DoubleEquals,
    Equals,
    Comma,
    At,
    Huh,
}
