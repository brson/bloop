#![allow(unused_imports)]

extern crate chrono;
extern crate failure;
extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate specs;
#[macro_use]
extern crate specs_derive;

mod lexer;

use failure::{err_msg, Error};
use std::env;
use std::fs::File;
use std::io::Read;
use std::result::Result as StdResult;

type Result<T> = StdResult<T, Error>;

fn main() -> StdResult<(), i32> {
    if let Err(e) = run() {
        println!("error: {}", e);
        println!("# {:#?}", e);
        for cause in e.iter_chain() {
            println!("  caused by: {}. i.e. \n", cause);
            println!("# {:#?}", cause);
        }
        Err(1)
    } else {
        Ok(())
    }
}

fn run() -> Result<()> {
    let file = env::args().skip(1).next().ok_or_else(|| err_msg("missing arg"))?;
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let _lexed = lexer::lex(&contents)?;
    
    Ok(())
}

/*struct UnresolvedWorldSeed;
struct WorldSeed;
struct WorldPrototype;

fn compile() -> Result<()> {
    let seed = UnresolvedWorldSeed;
    let seed: WorldSeed = resolve_seed(seed);
    let world_proto = resolve_world_proto(seed);

    let seed_tokens  lex_seed(seed);
    let seed_mod = parse_seed_mod(seed_tokens);
    let seed_mod_mapped = map_seed_mod_names(seed_mod);
    let main_tokens = get_main_tokens(seed_mod_mapped);

    let main = compile(world_proto);
}*/
