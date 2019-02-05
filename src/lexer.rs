use pest;

use failure::err_msg;
use failure::ResultExt;
use pest::Parser;

#[derive(Parser)]
#[grammar = "lexer.pest"]
struct Lexer;

use crate::Result;

pub fn lex(src: &str) -> Result<()> {
    let pairs = Lexer::parse(Rule::field, src)
        .context(format!("parsing source"))?;

    print!("num_pars: {}", pairs.clone().count());

    for pair in pairs {

        let span = pair.clone().into_span();
        // A pair is a combination of the rule which matched and a span of input
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", span);
        println!("Text:    {}", span.as_str());
        
    }

    Ok(())
}
