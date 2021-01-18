use b_base_analyzer_traits::BaseAnalyze;
use b_base_ast::BaseAst;
use b_deps::anyhow::Result;
use b_mir::Mir;

pub struct BaseAnalyzer;

impl BaseAnalyze for BaseAnalyzer {
    fn lower(&self, base_ast: BaseAst) -> Result<Mir> {
        Ok(Mir(base_ast))
    }
}
