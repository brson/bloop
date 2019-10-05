struct TokenTree(TreeOrThing);

enum TreeOrThing {
    Tree(Tree, Box<TreeOrThing>),
    Thing(Thing),
}

enum Tree {
    ParenTree,
    BraceTree,
    SquareTree,
    AngleTree,
}

enum Thing {
    Ident(Ident),
    Number(Number),
    Punctuation(Punctuation),
}

struct Ident(String);

enum Number {
    Float(Float),
    Int(Int),
}

struct Float(f64);
struct Int(u32);

enum Punctuation {
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
