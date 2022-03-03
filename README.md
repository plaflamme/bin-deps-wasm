# Cargo wasm binary dependency

The goal is to have a `bin` crate depend on a `.wasm` file that is built from a local dependency and only have to `cargo build` it.

To achieve this, it uses the [`bindeps`](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#artifact-dependencies) unstable feature.

This repository has the appropriate setup and `cargo build` does indeed succeed using nightly (`cargo 1.61.0-nightly (3d6970d50 2022-02-28)`).

An issue occurs when adding a (any?) dependency in the `wasm` module (see [`wasm-module/Cargo.toml`](wasm-module/Cargo.toml#L19)). This results in the following error:

```
thread 'main' panicked at 'activated_features for invalid package: features did not find PackageId { name: "log", version: "0.4.14", source: "registry `crates-io`" } NormalOrDevOrArtifactTarget(None)

Stack backtrace:
   0: std::backtrace::Backtrace::create
   1: std::backtrace::Backtrace::capture
   2: <anyhow::Error>::msg::<alloc::string::String>
   3: <cargo::core::resolver::features::ResolvedFeatures>::activated_features_int
   4: cargo::core::compiler::unit_dependencies::new_unit_dep_with_profile
   5: cargo::core::compiler::unit_dependencies::dep_build_script
   6: cargo::core::compiler::unit_dependencies::compute_deps
   7: cargo::core::compiler::unit_dependencies::deps_of
   8: cargo::core::compiler::unit_dependencies::deps_of
   9: cargo::core::compiler::unit_dependencies::deps_of
  10: cargo::core::compiler::unit_dependencies::deps_of_roots
  11: cargo::core::compiler::unit_dependencies::build_unit_dependencies
  12: cargo::ops::cargo_compile::create_bcx
  13: cargo::ops::cargo_compile::compile_ws
  14: cargo::ops::cargo_compile::compile
  15: cargo::commands::build::exec
  16: cargo::cli::main
  17: cargo::main
  18: std::sys_common::backtrace::__rust_begin_short_backtrace::<fn(), ()>
  19: std::rt::lang_start::<()>::{closure#0}
  20: std::rt::lang_start_internal
  21: _main', src/tools/cargo/src/cargo/core/resolver/features.rs:318:14
stack backtrace:
   0: _rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::result::unwrap_failed
   3: cargo::core::compiler::unit_dependencies::new_unit_dep_with_profile
   4: cargo::core::compiler::unit_dependencies::dep_build_script
   5: cargo::core::compiler::unit_dependencies::compute_deps
   6: cargo::core::compiler::unit_dependencies::deps_of
   7: cargo::core::compiler::unit_dependencies::deps_of
   8: cargo::core::compiler::unit_dependencies::deps_of
   9: cargo::core::compiler::unit_dependencies::deps_of_roots
  10: cargo::core::compiler::unit_dependencies::build_unit_dependencies
  11: cargo::ops::cargo_compile::create_bcx
  12: cargo::ops::cargo_compile::compile_ws
  13: cargo::ops::cargo_compile::compile
  14: cargo::commands::build::exec
  15: cargo::cli::main
  16: cargo::main
  ```
