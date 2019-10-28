#![allow(unused)]

use std::mem;
use std::fs;
use std::path::Path;
use b_codegen_traits::Codegen;
use b_error::{BResult, StdResultExt, BError};
use failure::ResultExt;
use b_mir::Mir;
use cranelift_codegen::ir::types;
use cranelift_codegen::ir::function::Function;
use cranelift_codegen::ir::{LibCall, AbiParam, ExternalName};
use cranelift_module::Backend;
use cranelift_module::{default_libcall_names, Linkage, Module};
use cranelift_simplejit::{SimpleJITBackend, SimpleJITBuilder};
use cranelift_frontend::{FunctionBuilderContext, FunctionBuilder};
use cranelift_codegen::ir::InstBuilder;
use b_codegen_traits::ExeOut;

pub struct CraneliftGenerator;

impl Codegen for CraneliftGenerator {
    fn emit_exe(&self, _mir: &Mir, _info: &ExeOut) -> BResult<()> {
        panic!()
    }

    fn jit(&self, _mir: &Mir) -> BResult<i32> {
        do_jit()
    }
}

fn do_jit() -> BResult<i32> {
    let builder = SimpleJITBuilder::new(default_libcall_names());
    let mut module: Module<SimpleJITBackend> =
        Module::new(builder);

    let mut ctx = module.make_context();
    let mut func_ctx = FunctionBuilderContext::new();

    let mut sig_a = module.make_signature();
    sig_a.returns.push(AbiParam::new(types::I32));

    let mut func_a = module
        .declare_function("a", Linkage::Local, &sig_a)
        .compat().e()?;

    {
        ctx.func.signature = sig_a;
        ctx.func.name = ExternalName::user(0, func_a.as_u32());

        let mut bcx: FunctionBuilder = FunctionBuilder::new(&mut ctx.func, &mut func_ctx);

        let ebb = bcx.create_ebb();

        bcx.switch_to_block(ebb);

        let value = bcx.ins().iconst(types::I32, 37);
        bcx.ins().return_(&[value]);
        bcx.seal_all_blocks();
        bcx.finalize();
        module.define_function(func_a, &mut ctx)
            .compat().e()?;
    }
    module.clear_context(&mut ctx);

    module.finalize_definitions();

    let code_a = module.get_finalized_function(func_a);
    let ptr_a = unsafe { mem::transmute::<_, fn() -> i32>(code_a) };

    let res = ptr_a();

    Ok(res)
}

fn make_main() -> Function {
    panic!()
}

pub struct ClifIR;

pub fn load_ir(path: &Path) -> BResult<ClifIR> {
    let irstr = fs::read_to_string(path).e()?;
    
    panic!()
}

pub fn jit_ir(ir: &ClifIR) -> BResult<()> {
    panic!()
}
