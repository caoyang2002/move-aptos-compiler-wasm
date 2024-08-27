# move-aptos-compiler-wasm

move compiler on Aptos (wasm version)

# Start

1. clone aptos-core.git

   ```bash
   git clone https://github.com/aptos-labs/aptos-core.git
   ```

2. install wasm-pack

   ```bash
   cargo install wasm-pack
   ```

   ```
   cd case
   npx http-server
   ```

   http://192.168.5.192:8080/move_wasm.html

3. compile

   ```bash
   wasm-pack build --target web
   ```

   ```bash
   $ wasm-pack build --target web
   [INFO]: 🎯  Checking for the Wasm target...
   [INFO]: 🌀  Compiling to Wasm...
   warning: unexpected `cfg` condition value: `serde1`
      --> /home/caoyang/wasm/aptos_move_wasm/aptos-core/third_party/move/move-core/types/src/u256.rs:570:12
       |
   570 | #[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
       |            ^^^^^^^^^^^^^^^^^^
       |
       = note: expected values for `feature` are: `arbitrary`, `default`, `fuzzing`, `proptest`, and `proptest-derive`
       = help: consider adding `serde1` as a feature in `Cargo.toml`
       = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration
       = note: `#[warn(unexpected_cfgs)]` on by default

   warning: `move-core-types` (lib) generated 1 warning
   warning: unexpected `cfg` condition value: `evm-backend`
       --> /home/caoyang/wasm/aptos_move_wasm/aptos-core/third_party/move/move-compiler/src/to_bytecode/translate.rs:1141:36
        |
   1141 |         Some(mk_bytecode) if !cfg!(feature = "evm-backend") => {
        |                                    ^^^^^^^^^^^^^^^^^^^^^^^ help: remove the condition
        |
        = note: no expected values for `feature`
        = help: consider adding `evm-backend` as a feature in `Cargo.toml`
        = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration
        = note: `#[warn(unexpected_cfgs)]` on by default

   warning: field `env` is never read
     --> /home/caoyang/wasm/aptos_move_wasm/aptos-core/third_party/move/move-compiler/src/to_bytecode/context.rs:32:9
      |
   31 | pub struct Context<'a> {
      |            ------- field in this struct
   32 |     pub env: &'a mut CompilationEnv,
      |         ^^^
      |
      = note: `#[warn(dead_code)]` on by default

   warning: field `inline` is never read
     --> /home/caoyang/wasm/aptos_move_wasm/aptos-core/third_party/move/move-compiler/src/typing/core.rs:45:9
      |
   40 | pub struct FunctionInfo {
      |            ------------ field in this struct
   ...
   45 |     pub inline: bool,
      |         ^^^^^^

   warning: `move-compiler` (lib) generated 3 warnings
      Compiling move-compiler-wasm v0.1.0 (/home/caoyang/wasm/aptos_move_wasm/move-compiler-wasm)
   warning: unused import: `move_compiler::*`
    --> src/lib.rs:2:5
     |
   2 | use move_compiler::*;
     |     ^^^^^^^^^^^^^^^^
     |
     = note: `#[warn(unused_imports)]` on by default

   warning: unused variable: `source`
    --> src/lib.rs:5:21
     |
   5 | pub fn compile_move(source: &str) -> String {
     |                     ^^^^^^ help: if this is intentional, prefix it with an underscore: `_source`
     |
     = note: `#[warn(unused_variables)]` on by default

   warning: `move-compiler-wasm` (lib) generated 2 warnings (run `cargo fix --lib -p move-compiler-wasm` to apply 1 suggestion)
       Finished `release` profile [optimized] target(s) in 1.69s
   [INFO]: ⬇️  Installing wasm-bindgen...
   [INFO]: Optimizing wasm binaries with `wasm-opt`...
   [INFO]: Optional fields missing from Cargo.toml: 'description', 'repository', and 'license'. These are not necessary, but recommended
   [INFO]: ✨   Done in 6m 59s
   [INFO]: 📦   Your wasm pkg is ready to publish at /home/caoyang/wasm/aptos_move_wasm/move-compiler-wasm/pkg.

   $ ls
   Cargo.lock  Cargo.toml  pkg  src  target

   $ cd pkg/ && ls
   move_compiler_wasm.d.ts  move_compiler_wasm.js  move_compiler_wasm_bg.wasm  move_compiler_wasm_bg.wasm.d.ts  package.json
   ```

# Use
