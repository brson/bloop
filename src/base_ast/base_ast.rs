#[derive(Debug)]
pub struct BaseAst(pub Module);

#[derive(Debug)]
pub struct Module {
    pub decls: Vec<Declaration>,
}

#[derive(Debug)]
pub enum Declaration {
    Function(Function),
}

#[derive(Debug)]
pub struct Function {
    pub name: Ident,
    pub args: ArgList,
    pub ret: RetDecl,
    pub body: Body,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Ident(pub String);

#[derive(Debug)]
pub struct ArgList(pub Vec<Argument>);

#[derive(Debug)]
pub struct RetDecl(pub Type);

#[derive(Debug)]
pub struct Argument {
    pub name: Ident,
    pub type_: Type,
}

#[derive(Debug)]
pub struct Body {
    pub stmts: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Let(Let),
    Const(Const),
    Return(Ident),
}

#[derive(Debug)]
pub struct Let {
    pub name: Ident,
    pub type_: Type,
    pub lit: Literal,
}

#[derive(Debug)]
pub struct Const {
    pub name: Ident,
    pub type_: Type,
    pub lit: Literal,
}

#[derive(Debug)]
pub enum Literal {
    Int32(String),
}

#[derive(Debug)]
pub enum Type {
    Int32,
}
