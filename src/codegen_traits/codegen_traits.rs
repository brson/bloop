use std::path::PathBuf;
use b_error::BResult;
use b_mir::Mir;

pub trait Codegen {
    fn emit_exe(&self, mir: &Mir, info: &ExeOut) -> BResult<()>;
    fn jit(&self, mir: &Mir) -> BResult<i32>;
}

pub struct ExeOut {
    pub path: PathBuf,
}
