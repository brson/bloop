static KEYWORDS: &[&str] = &[
    "break",
    "continue",
    "fn",
    "if",
    "impl",
    "let",
    "loop",
    "match",
    "move",
    "newtype",
    "priv",
    "pub",
    "return",
    "self",
    "type",
    "typedef",
    "while",
];

enum File {
    Module(Module),
}

struct Module(Vec<Item>);

enum Item {
    Import(Import),
    Namespace(Namespace),
    Function(Function),
    Impl(Impl),
}

struct Import;
struct Namespace;

struct Function {
    vis: Visibility,
    name: Ident,
    params: Vec<Parameter>,
    return_: Type,
    body: Body,
}

enum Visibility {
    Public, Private
}

struct Impl;

struct Ident(String);

struct Parameter {
    name: Ident,
    type_: Type,
}

struct Type(Path);

struct Path(Vec<Ident>);

struct Body(Block);

struct Block {
    stmts: Vec<Statement>,
    tail: Option<Expr>,
}

struct Statement(Expr);

enum Expr {
    Let,
    If,
    Match,
    Loop,
}
