use b_codegen_traits::Codegen;
use b_error::BResult;
use b_mir::Mir;

pub struct CraneliftGenerator;

impl Codegen for CraneliftGenerator {
    fn jit(&self, _mir: &Mir) -> BResult<i32> {
        panic!()
    }
}
