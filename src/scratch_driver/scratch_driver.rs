extern crate env_logger;

use b_deps::log::debug;
use b_base_analyzer::BaseAnalyzer;
use b_base_analyzer_traits::BaseAnalyze;
use b_base_parser::BaseParser;
use b_base_parser_traits::BaseParse;
use b_codegen_cranelift::CraneliftGenerator;
use b_codegen_traits::Codegen;
use b_lexer::Lexer;
use b_lexer_traits::Lex;
use b_deps::anyhow::{Result, Context};

use std::fs::File;
use std::io::Read;
use std::path::{PathBuf, Path};
use structopt::StructOpt;
use std::process;

fn main() {
    main_error(run)
}

pub fn main_error(run: impl FnOnce() -> Result<()>) {
    if let Err(e) = run() {
        println!("error: {}", e);
        let mut e = e.source();
        while let Some(cause) = e {
            println!("  caused by: {}", cause);
            e = cause.source();
        }
        process::exit(1);
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
        Mode::LexDump(m) => run_lex_dump(m),
        Mode::BenchLex(m) => bench_lex(m),
        Mode::ParseBaseLang(m) => run_parse_baselang(m),
        Mode::ParseBaseLangPartial(m) => run_parse_baselang_partial(m),
        Mode::JitBaseLang(m) => run_jit_baselang(m),
        Mode::JitCranelift(m) => run_jit_cranelift(m),
    }
}

fn read_source(file: &Path) -> Result<String> {
    let mut contents = String::new();
    let mut file = File::open(file)
        .context("opening source file")?;
    file.read_to_string(&mut contents)
        .context("reading source as string")?;

    Ok(contents)
}

fn run_lex_dump(opts: LexDumpOpts) -> Result<()> {
    let lexer = Box::new(Lexer) as Box<dyn Lex>;

    let source = read_source(&opts.file)?;
    let token_tree = lexer.lex(&source)?;

    print!("tt: {:#?}", token_tree);
    
    Ok(())
}

fn bench_lex(opts: BenchLexOpts) -> Result<()> {
    let lexer = Box::new(Lexer) as Box<dyn Lex>;

    let source = read_source(&opts.file)?;
    lexer.lex(&source)?;
    
    Ok(())
}

fn run_parse_baselang(opts: ParseBaseLangOpts) -> Result<()> {
    let lexer = Box::new(Lexer) as Box<dyn Lex>;
    let base_parser = Box::new(BaseParser) as Box<dyn BaseParse>;
    
    let source = read_source(&opts.file)?;
    debug!("src: {:?}", source);
    let token_tree = lexer.lex(&source)?;
    debug!("tt: {:#?}", token_tree);
    let ast = base_parser.parse(&token_tree)?;
    debug!("ast: {:#?}", ast);

    Ok(())
}

fn run_parse_baselang_partial(opts: ParseBaseLangPartialOpts) -> Result<()> {
    use b_base_parser_lalrpop::parse_module;
    
    let lexer = Box::new(Lexer) as Box<dyn Lex>;
    
    let source = read_source(&opts.file)?;
    debug!("src: {:?}", source);
    let token_tree = lexer.lex(&source)?;
    debug!("tt: {:#?}", token_tree);
    let ast = parse_module(&token_tree)?;
    debug!("ast: {:#?}", ast);

    Ok(())
}

fn run_jit_baselang(opts: JitBaseLangOpts) -> Result<()> {
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
    let mir = base_analyzer.lower(ast)?;
    debug!("mir: {:#?}", mir);

    let retcode = codegen.jit(&mir)?;

    debug!("retcode: {}", retcode);

    process::exit(retcode);
}

fn run_jit_cranelift(opts: JitCraneliftOpts) -> Result<()> {
    use b_codegen_cranelift as cl;

    let ir = cl::load_ir(&opts.file)?;
    cl::jit_ir(&ir)?;

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
    #[structopt(name = "bench-lex")]
    BenchLex(BenchLexOpts),
    #[structopt(name = "parse-baselang")]
    ParseBaseLang(ParseBaseLangOpts),
    #[structopt(name = "parse-baselang-partial")]
    ParseBaseLangPartial(ParseBaseLangPartialOpts),
    #[structopt(name = "jit-baselang")]
    JitBaseLang(JitBaseLangOpts),
    #[structopt(name = "jit-cranelift")]
    JitCranelift(JitCraneliftOpts),
}

#[derive(Debug, StructOpt)]
struct LexDumpOpts {
    #[structopt(name = "file")]
    file: PathBuf,
}

#[derive(Debug, StructOpt)]
struct BenchLexOpts {
    #[structopt(name = "file")]
    file: PathBuf,
}

#[derive(Debug, StructOpt)]
struct ParseBaseLangOpts {
    #[structopt(name = "file")]
    file: PathBuf,
}

#[derive(Debug, StructOpt)]
struct ParseBaseLangPartialOpts {
    #[structopt(name = "file")]
    file: PathBuf,
}

#[derive(Debug, StructOpt)]
struct JitBaseLangOpts {
    #[structopt(name = "file")]
    file: PathBuf,
}

#[derive(Debug, StructOpt)]
struct JitCraneliftOpts {
    #[structopt(name = "file")]
    file: PathBuf,
}
