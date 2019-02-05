use pest;

use failure::err_msg;
use failure::ResultExt;
use pest::Parser;

#[derive(Parser)]
#[grammar = "lexer.pest"]
struct Lexer;

use crate::Result;

pub fn lex(src: &str) -> Result<()> {
    println!("source:\n{}", src);

    let mut pairs = Lexer::parse(Rule::file, src)
        .context(format!("parsing source"))?;

    println!("num_pars: {}", pairs.clone().count());
    assert!(pairs.clone().count() == 1); // FIXME

    let file = pairs.next().unwrap();

    let mut num_records = 0;
    let mut field_sum = 0.0;
    for record in file.into_inner() {
        println!("foo");
        match record.as_rule() {
            Rule::record => {
                num_records += 1;

                for field in record.into_inner() {
                    println!("bar");
                    field_sum += field.as_str().parse::<f64>()
                        .context("parsing number")?;
                }
            },
            Rule::EOI => { }
            e => { panic!("bad record: {:?}", e); }
        }
    }

    println!("num_records: {}", num_records);
    println!("field_sum: {}", field_sum);

    Ok(())
}
