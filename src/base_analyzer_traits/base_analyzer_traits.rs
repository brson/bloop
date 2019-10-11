use b_base_ast::BaseAst;
use b_error::BResult;
use b_mir::Mir;

pub trait BaseAnalyze {
    fn lower(&self, base_ast: &BaseAst) -> BResult<Mir>;
}


    
