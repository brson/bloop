use b_deps::anyhow::Result;
use b_token_tree::TokenTree;

pub trait Lex {
    fn lex(&self, src: &str) -> Result<TokenTree>;
}
