pub struct TokenTree(Vec<TreeOrThing>);

pub enum TreeOrThing {
    Tree(Tree, TokenTree),
    Thing(Thing),
}

pub enum Tree {
    ParenTree,
    BraceTree,
    SquareTree,
    AngleTree,
}

pub enum Thing {
    Ident(Ident),
    Number(Number),
    Punctuation(Punctuation),
}

pub struct Ident(String);

pub enum Number {
    Float(Float),
    UInt(UInt),
}

pub struct Float(pub String);
pub struct UInt(pub String);

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
