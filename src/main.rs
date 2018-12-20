#![allow(unused_imports)]

extern crate chrono;
extern crate failure;
extern crate pest;
#[macro_use]
extern crate pest_derive;

mod lexer;
mod lexer2;

use failure::{err_msg, Error};
use std::env;
use std::fs::File;
use std::io::Read;

type Result<T> = std::result::Result<T, Error>;

fn main() {
    run().unwrap();
}

fn run() -> Result<()> {
    let file = env::args().skip(1).next().ok_or_else(|| err_msg("missing arg"))?;
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let _ = lexer2::lex(&contents);
    
    Ok(())
}

#[allow(unused)]
mod compiler {
    use std::path::Path;
    use crate::Result;

    pub type DateTime = ::chrono::DateTime<::chrono::Utc>;
    pub type Duration = ::chrono::Duration;
    pub type Ident = String;
    pub type Name = Vec<Ident>;

    pub struct Ast;

    pub enum MomentTime {
        Precise(DateTime),
        After(String, Duration),
        Whenever,
    }

    pub struct Moment {
        name: Name,
        time: MomentTime,
        preqs: Vec<Name>,
    }

    pub fn parse(path: &Path) -> Result<Ast> {
        panic!()
    }

    pub fn ast_to_moments(ast: &Ast) -> Result<Vec<Moment>> {
        panic!()
    }

    pub fn id_moments(moments: Vec<Moment>) -> Result<Vec<Moment>> {
        panic!()
    }

    pub fn sort_moments(moments: Vec<Moment>) -> Result<Vec<Moment>> {
        panic!()
    }
}

mod runtime {
}
