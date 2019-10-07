#[derive(Debug)]
pub struct TokenTree(pub Vec<TreeOrThing>);

#[derive(Debug)]
pub enum TreeOrThing {
    Tree(Tree, TokenTree),
    Thing(Thing),
}

#[derive(Debug)]
pub enum Tree {
    ParenTree,
    BraceTree,
    SquareTree,
    AngleTree,
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
    UInt(UInt),
}

#[derive(Debug)]
pub struct Float(pub String);
#[derive(Debug)]
pub struct UInt(pub String);

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
