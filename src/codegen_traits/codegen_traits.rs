use b_error::BResult;
use b_mir::Mir;

pub trait Codegen {
    fn jit(&self, mir: &Mir) -> BResult<i32>;
}
