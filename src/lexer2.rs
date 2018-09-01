use pest;

use failure::err_msg;
use pest::Parser;

#[derive(Parser)]
#[grammar = "lexer.pest"]
struct Lexer;

use Result;

pub fn lex(src: &str) -> Result<()> {
    println!("foo");
    let pairs = match Lexer::parse(Rule::document, src) {
        Ok(p) => p,
        Err(e) => {
            return Err(err_msg(format!("{}", e)));
        }
    };
    println!("bar");

    for pair in pairs {

        let span = pair.clone().into_span();
        // A pair is a combination of the rule which matched and a span of input
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", span);
        println!("Text:    {}", span.as_str());
        
    }

    Ok(())
}
