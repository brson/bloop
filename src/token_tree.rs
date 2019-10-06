pub struct Module(Vec<TreeOrThing>);

pub struct TokenTree(TreeOrThing);

pub enum TreeOrThing {
    Tree(Tree, Vec<TreeOrThing>),
    Thing(Vec<Thing>),
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
    Int(Int),
}

pub struct Float(f64);
pub struct Int(u32);

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
