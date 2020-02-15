use b_base_analyzer_traits::BaseAnalyze;
use b_base_ast::BaseAst;
use b_error::BResult;
use b_mir::Mir;

pub struct BaseAnalyzer;

impl BaseAnalyze for BaseAnalyzer {
    fn lower(&self, _base_ast: BaseAst) -> BResult<Mir> {
        Ok(Mir)
    }
}
