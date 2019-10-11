use b_base_ast::BaseAst;
use b_base_parser_traits::BaseParse;
use b_error::BResult;
use b_token_tree::TokenTree;

pub struct BaseParser;

impl BaseParse for BaseParser {
    fn parse(&self, _tt: &TokenTree) -> BResult<BaseAst> {
        panic!()
    }
}
