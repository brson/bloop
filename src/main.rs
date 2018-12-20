#![allow(unused_imports)]

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
    
