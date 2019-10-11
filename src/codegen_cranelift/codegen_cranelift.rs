use b_codegen_traits::Gen;
use b_error::BResult;
use b_mir::Mir;

pub struct CraneliftGen;

impl Gen for CraneliftGen {
    fn jit(mir: &Mir) -> BResult<i32> {
        panic!()
    }
}
