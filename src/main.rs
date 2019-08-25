#![allow(unused)]

#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate specs_derive;
#[macro_use]
extern crate structopt;
#[macro_use]
extern crate log;
extern crate env_logger;

mod lexer;

use failure::{err_msg, Error};
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::result::Result as StdResult;
use structopt::StructOpt;

type Result<T> = StdResult<T, Error>;

fn main() -> StdResult<(), i32> {
    if let Err(e) = run() {
        println!("error: {}", e);
        println!("# {}", e);
        for cause in e.iter_chain() {
            println!("  caused by: {}. i.e. \n", cause);
            println!("# {}", cause);
        }
        Err(1)
    } else {
        Ok(())
    }
}

fn run() -> Result<()> {
    use env_logger::Builder;

    Builder::from_default_env()
        .default_format_timestamp(false)
        .init();

    let opts = Opts::from_args();
    dispatch_command(opts)?;
    Ok(())
}

fn dispatch_command(opts: Opts) -> Result<()> {
    debug!("command line options: {:#?}", opts);

    match opts.mode {
        Mode::DoThing(m) => run_do_thing(m),
    }
}

mod dataflow;

fn run_do_thing(opts: DoThingOpts) -> Result<()> {
    dataflow::do_your_thing()?;

    let mut file = File::open(&opts.root_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let _lexed = lexer::lex(&contents)?;
    
    Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt(name = "bloop")]
struct Opts {
    #[structopt(subcommand)]
    mode: Mode,
}

#[derive(Debug, StructOpt)]
enum Mode {
    #[structopt(name = "dothing")]
    DoThing(DoThingOpts),
}

#[derive(Debug, StructOpt)]
struct DoThingOpts {
    #[structopt(name = "file")]
    root_path: PathBuf,
}
    
