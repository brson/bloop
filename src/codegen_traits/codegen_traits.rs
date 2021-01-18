use std::path::PathBuf;
use b_deps::anyhow::Result;
use b_mir::Mir;

pub trait Codegen {
    fn emit_exe(&self, mir: &Mir, info: &ExeOut) -> Result<()>;
    fn jit(&self, mir: &Mir) -> Result<i32>;
}

pub struct ExeOut {
    pub path: PathBuf,
}
