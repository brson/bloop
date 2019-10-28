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
    pub name: String,
    pub args: Vec<Argument>,
    pub ret: Type,
    pub body: Body,
}

#[derive(Debug)]
pub struct Argument {
}

#[derive(Debug)]
pub struct Body {
    pub stmts: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Const(Const),
}

#[derive(Debug)]
pub struct Const {
    pub name: String,
    pub type_: Type,
    pub lit: Literal,
}

#[derive(Debug)]
pub enum Literal {
    I32(i32),
}

#[derive(Debug)]
pub enum Type {
    I32,
}
