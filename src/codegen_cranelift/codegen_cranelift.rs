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
use b_base_ast::{Declaration, Type, Statement, Literal};
use std::collections::HashMap;

pub struct CraneliftGenerator;

impl Codegen for CraneliftGenerator {
    fn emit_exe(&self, _mir: &Mir, _info: &ExeOut) -> BResult<()> {
        panic!()
    }

    fn jit(&self, mir: &Mir) -> BResult<i32> {
        do_jit(mir)
    }
}

fn do_jit(mir: &Mir) -> BResult<i32> {
    let builder = SimpleJITBuilder::new(default_libcall_names());
    let mut module: Module<SimpleJITBackend> =
        Module::new(builder);

    let mut ctx = module.make_context();

    let ast_mod = &(mir.0).0;

    let mut main = None;

    for ast_decl in &ast_mod.decls {

        match ast_decl {
            Declaration::Function(ref fn_) => {

                let cl_fn_name = &fn_.name.0;
                let cl_ret_ty = ty_to_cl_ty(&fn_.ret.0);

                let mut func_ctx = FunctionBuilderContext::new();

                let mut sig_a = module.make_signature();

                sig_a.returns.push(AbiParam::new(cl_ret_ty));

                let mut func_a = module
                    .declare_function(cl_fn_name, Linkage::Local, &sig_a)
                    .compat().e()?;

                {
                    ctx.func.signature = sig_a;
                    ctx.func.name = ExternalName::user(0, func_a.as_u32());

                    let mut bcx: FunctionBuilder = FunctionBuilder::new(&mut ctx.func, &mut func_ctx);

                    let ebb = bcx.create_ebb();

                    bcx.switch_to_block(ebb);

                    let mut iconsts = HashMap::new();

                    for stmt in &fn_.body.stmts {

                        match stmt {
                            Statement::Let(ref let_) => {
                                let ty = ty_to_cl_ty(&let_.type_);
                                let val = lit_to_iconst(&let_.lit)?;
                                let value = bcx.ins().iconst(ty, val);
                                iconsts.insert(let_.name.clone(), value);
                            }
                            Statement::Const(ref const_) => {
                                unimplemented!()
                            }
                            Statement::Return(ref ident) => {
                                let value = iconsts.get(ident)
                                    .ok_or_else(|| BError::new("undefined ident"))?;
                                bcx.ins().return_(&[*value]);
                            }
                        }
                    }

                    bcx.seal_all_blocks();
                    bcx.finalize();

                    module.define_function(func_a, &mut ctx)
                        .compat().e()?;

                    if cl_fn_name == "main" {
                        main = Some(func_a);
                    }
                }
            }
        }

        module.clear_context(&mut ctx);
    }

    module.finalize_definitions();

    if let Some(main) = main {
        let code_a = module.get_finalized_function(main);
        let ptr_a = unsafe { mem::transmute::<_, fn() -> i32>(code_a) };

        let res = ptr_a();

        Ok(res)
    } else {
        return Err(BError::new("no main function declared"));
    }
}

fn ty_to_cl_ty(ty: &Type) -> types::Type {
    match ty {
        Type::Int32 => types::I32,
    }
}

fn lit_to_iconst(lit: &Literal) -> BResult<i64> {
    match lit {
        Literal::Int32(s) => {
            // FIXME this is lame and belongs somewhere else
            assert!(s.ends_with("_i32"));
            let s = &s[..s.len()-4];
            s.parse().e()
        }
    }
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
