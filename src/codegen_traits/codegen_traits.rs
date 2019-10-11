use b_error::BResult;
use b_mir::Mir;

pub trait Gen {
    fn jit(mir: &Mir) -> BResult<i32>;
}
