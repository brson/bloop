#[macro_use]
extern crate log;
extern crate env_logger;

use b_lexer;
use b_error::BResult;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::result::Result as StdResult;
use structopt::StructOpt;

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

fn run() -> BResult<()> {
    use env_logger::Builder;

    Builder::from_default_env()
        .default_format_timestamp(false)
        .init();

    let opts = Opts::from_args();
    dispatch_command(opts)?;
    Ok(())
}

fn dispatch_command(opts: Opts) -> BResult<()> {
    debug!("command line options: {:#?}", opts);

    match opts.mode {
        Mode::LexDump(m) => run_lex_dump(m),
    }
}

fn run_lex_dump(opts: LexDumpOpts) -> BResult<()> {
    let mut file = File::open(&opts.file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let token_tree = b_lexer::lex(&contents)?;

    print!("tt: {:#?}", token_tree);
    
    Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt(name = "bloop-scratch")]
struct Opts {
    #[structopt(subcommand)]
    mode: Mode,
}

#[derive(Debug, StructOpt)]
enum Mode {
    #[structopt(name = "lex-dump")]
    LexDump(LexDumpOpts),
}

#[derive(Debug, StructOpt)]
struct LexDumpOpts {
    #[structopt(name = "file")]
    file: PathBuf,
}
    
