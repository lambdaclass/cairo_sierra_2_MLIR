# Cairo Sierra to MLIR compiler
[![test](https://github.com/lambdaclass/cairo_sierra_to_mlir/actions/workflows/test.yml/badge.svg)](https://github.com/lambdaclass/cairo_sierra_to_mlir/actions/workflows/test.yml)

A compiler to convert Cairo's intermediate representation "Sierra" code to MLIR.

## Dependencies

- LLVM 16+ with MLIR
- Rust

## CLI Interface

```
Usage: cli --input <INPUT> <COMMAND>

Commands:
  compile  Compile to MLIR with LLVM dialect, ready to be converted by `mlir-translate --mlir-to-llvmir`
  run      Compile and run a program. The entry point must be a function without arguments
  help     Print this message or the help of the given subcommand(s)

Options:
  -i, --input <INPUT>  The input sierra file
  -h, --help           Print help (see more with '--help')
  -V, --version        Print version
```

## Setup

Install LLVM with MLIR

```bash
wget https://github.com/llvm/llvm-project/releases/download/llvmorg-16.0.0-rc3/llvm-project-16.0.0rc3.src.tar.xz

mkdir ~/mlir
tar -xf llvm-project-16.0.0rc3.src.tar.xz
cd llvm-project-16.0.0rc3.src.tar
mkdir build
cd build

cmake -G Ninja ../llvm \
   -DLLVM_ENABLE_PROJECTS=mlir \
   -DLLVM_BUILD_EXAMPLES=ON \
   -DLLVM_TARGETS_TO_BUILD="X86;AArch64;NVPTX;AMDGPU" \
   -DCMAKE_BUILD_TYPE=RelWithDebInfo \
   -DLLVM_ENABLE_ASSERTIONS=ON \
   -DLLVM_INSTALL_UTILS=ON \
   -DCMAKE_C_COMPILER=clang -DCMAKE_CXX_COMPILER=clang++ -DLLVM_ENABLE_LLD=ON \
   -DCMAKE_INSTALL_PREFIX=~/mlir
```

Setup a environment variable called `MLIR_SYS_160_PREFIX` pointing to the mlir directory:

```bash
MLIR_SYS_160_PREFIX=~/mlir
```

## Benchmarking

```bash
cargo bench
```

## MLIR Resources
- https://mlir.llvm.org/docs/Tutorials/

## Translate output MLIR to LLVM IR

```
mlir-translate --mlir-to-llvmir output.mlir -o output.ll

# Compile with clang
clang -O3 output.ll -o program
./program

# With JIT
lli output.ll
```
