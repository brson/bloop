use b_base_ast::BaseAst;
use b_deps::anyhow::Result;
use b_mir::Mir;

pub trait BaseAnalyze {
    fn lower(&self, base_ast: BaseAst) -> Result<Mir>;
}


    
