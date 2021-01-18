use b_base_ast::BaseAst;
use b_deps::anyhow::Result;
use b_token_tree::TokenTree;

pub trait BaseParse {
    fn parse(&self, tt: &TokenTree) -> Result<BaseAst>;
}
