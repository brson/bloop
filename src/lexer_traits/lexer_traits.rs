use b_error::BResult;
use b_token_tree::TokenTree;

pub trait Lex {
    fn lex(&self, src: &str) -> BResult<TokenTree>;
}
