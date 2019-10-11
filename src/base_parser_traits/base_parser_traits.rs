use b_base_ast::BaseAst;
use b_error::BResult;
use b_token_tree::TokenTree;

pub trait BaseParse {
    fn parse(&self, tt: &TokenTree) -> BResult<BaseAst>;
}
