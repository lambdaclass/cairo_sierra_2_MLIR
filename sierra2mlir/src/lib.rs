//! A compiler to convert Cairo's intermediate representation "Sierra" code to MLIR.

//#![deny(warnings)]
#![warn(clippy::nursery)]
#![allow(unused)]

use std::{fs, path::PathBuf, time::Instant};

use cairo_lang_sierra::{program::Program, ProgramParser};
use melior_next::{
    dialect,
    ir::{operation, Attribute, Block, Identifier, Location, Module, Region, Type, Value},
    pass,
    utility::{register_all_dialects, register_all_passes},
    Context, ExecutionEngine,
};

use crate::compiler::Compiler;

pub mod compiler;
mod libfuncs;
mod statements;
mod types;

pub fn compile(code: &str) -> Result<String, color_eyre::Report> {
    let mut compiler = Compiler::new(code)?;
    compiler.compile()?;

    let pass_manager = pass::Manager::new(&compiler.context);
    register_all_passes();
    pass_manager.add_pass(pass::conversion::convert_scf_to_cf());
    pass_manager.add_pass(pass::conversion::convert_cf_to_llvm());
    //pass_manager.add_pass(pass::conversion::convert_gpu_to_llvm());
    pass_manager.add_pass(pass::conversion::convert_arithmetic_to_llvm());
    pass_manager.enable_verifier(true);
    pass_manager.run(&mut compiler.module)?;

    let op = compiler.module.as_operation();
    if op.verify() {
        Ok(op.to_string())
    } else {
        Err(color_eyre::eyre::eyre!("error verifiying"))
    }
}

pub fn execute(code: &str) -> Result<ExecutionEngine, color_eyre::Report> {
    let mut compiler = Compiler::new(code)?;
    compiler.compile()?;

    let pass_manager = pass::Manager::new(&compiler.context);
    register_all_passes();
    pass_manager.add_pass(pass::conversion::convert_scf_to_cf());
    pass_manager.add_pass(pass::conversion::convert_cf_to_llvm());
    //pass_manager.add_pass(pass::conversion::convert_gpu_to_llvm());
    pass_manager.add_pass(pass::conversion::convert_arithmetic_to_llvm());
    pass_manager.enable_verifier(true);
    pass_manager.run(&mut compiler.module)?;

    let engine = ExecutionEngine::new(&compiler.module, 2, &[], false);

    Ok(engine)
}
