#[macro_use]
extern crate log;
extern crate env_logger;

use b_base_analyzer::BaseAnalyzer;
use b_base_analyzer_traits::BaseAnalyze;
use b_base_parser::BaseParser;
use b_base_parser_traits::BaseParse;
use b_codegen_cranelift::CraneliftGenerator;
use b_codegen_traits::Codegen;
use b_lexer::Lexer;
use b_lexer_traits::Lex;
use b_error::{BResult, ResultExt};

use std::fs::File;
use std::io::Read;
use std::path::{PathBuf, Path};
use structopt::StructOpt;

fn main() {
    b_error::main(run)
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
        Mode::JitBaseLang(m) => run_jit_baselang(m),
    }
}

fn read_source(file: &Path) -> BResult<String> {
    let mut contents = String::new();
    let mut file = File::open(file)
        .ec("opening source file")?;
    file.read_to_string(&mut contents)
        .ec("reading source as string")?;

    Ok(contents)
}

fn run_lex_dump(opts: LexDumpOpts) -> BResult<()> {
    let lexer = Box::new(Lexer) as Box<dyn Lex>;

    let source = read_source(&opts.file)?;
    let token_tree = lexer.lex(&source)?;

    print!("tt: {:#?}", token_tree);
    
    Ok(())
}

fn run_jit_baselang(opts: JitBaseLangOpts) -> BResult<()> {
    let lexer = Box::new(Lexer) as Box<dyn Lex>;
    let base_parser = Box::new(BaseParser) as Box<dyn BaseParse>;
    let base_analyzer = Box::new(BaseAnalyzer) as Box<dyn BaseAnalyze>;
    let codegen = Box::new(CraneliftGenerator) as Box<dyn Codegen>;
    
    let source = read_source(&opts.file)?;
    debug!("src: {:?}", source);
    let token_tree = lexer.lex(&source)?;
    debug!("tt: {:#?}", token_tree);
    let ast = base_parser.parse(&token_tree)?;
    debug!("ast: {:#?}", ast);
    let mir = base_analyzer.lower(&ast)?;
    debug!("mir: {:#?}", mir);

    codegen.jit(&mir)?;

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
    #[structopt(name = "jit-baselang")]
    JitBaseLang(JitBaseLangOpts),
}

#[derive(Debug, StructOpt)]
struct LexDumpOpts {
    #[structopt(name = "file")]
    file: PathBuf,
}

#[derive(Debug, StructOpt)]
struct JitBaseLangOpts {
    #[structopt(name = "file")]
    file: PathBuf,
}

