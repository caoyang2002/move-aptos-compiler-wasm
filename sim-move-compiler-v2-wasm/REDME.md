# Move wasm compiler v2 test

在 [`move-compiler/src/bin/`](./move-compiler/src/bin/) 中两个文件都存在 `main()` 函数 

## 运行

```bash
cd move-compiler

#cargo run --bin move-check

cargo run --bin move-check -- --flavor  tests/move_check_testsuite.rs
```

`cargo run --bin move-check` 是一个用于运行 Rust 语言编写的 `move-check` 二进制程序的命令。这个命令的运行过程可以分为以下几个步骤：

1. 查找项目下的 [Cargo.toml](./move-compiler/Cargo.toml) 文件：
   - `cargo run` 命令首先会在当前目录及其子目录中查找 `Cargo.toml` 文件，这个文件包含了项目的配置和依赖信息。

2. 解析项目配置：
   - 找到 `Cargo.toml` 文件后，Cargo（Rust 的包管理器和构建工具）会解析文件中的配置，包括项目名称、版本、依赖关系等。

3. 编译项目：
   - 如果项目还没有编译过，或者源代码有更新，Cargo 会先编译项目。这包括下载依赖、编译项目中的每个包（`crate`），以及链接生成最终的可执行文件。

4. 查找二进制目标：
   - 编译完成后，Cargo 会在 `target` 目录下的相应配置文件夹（如 `debug` 或 `release`）中查找名为 [`move-check`](./target/debug/move-check) 的二进制文件。

5. 运行二进制文件：
   - 找到 `move-check` 可执行文件后，Cargo 会执行这个文件。如果 `move-check` 需要命令行参数，你需要在 `cargo run --bin move-check` 后面提供这些参数。

6. **执行命令**：
   - `move-check` 程序会根据提供的参数执行相应的操作。例如，检查 Move 源代码是否符合语法规范，但不生成字节码。

7. **处理输出**：
   - 程序执行完毕后，会将任何输出（包括错误信息、警告等）显示在终端上。

请注意，`cargo run --bin move-check` 默认会以 `dev` 配置运行程序，这意味着程序是未优化的，并且包含了调试信息。如果你想以不同的配置运行程序，比如优化过的 `release` 配置，你可以使用 `cargo run --bin move-check --release` 命令。

如果你在运行 `move-check` 时遇到了错误，比如缺少 `--flavor` 参数，你需要根据错误信息提供的用法提示，添加必要的参数来正确执行命令。

## 获取 move-check 命令可用项

```bash
cargo run --bin move-check -- --help
```

```bash
警告：`move-compiler`（库）生成了 6 个警告
    在 1.30 秒内完成 `dev` 配置文件[未优化 + 调试信息]目标(s)
     运行 `/Users/simons/Documents/GitHub/move-aptos-compiler-wasm/aptos-move-compiler-v2-wasm/target/debug/move-check --help`
检查 Move 源代码，不编译成字节码

用法：move-check [选项] --flavor <风格> [源文件路径]...

参数：
  [源文件路径]...  要检查的源文件

选项：
  -d, --dependency <依赖文件路径>    # 所需的库文件作为依赖项
  -o, --out-dir <输出目录路径>       # 保存生成的工件的输出目录，即从 'mv' 文件生成的任何 'move' 接口文件
  -a, --addresses <命名地址>        # 命名地址映射
  -t, --test                       # 以测试模式编译
  -v, --verify                     # 以验证模式编译
      --flavor <风格>               # 编译风格
      --bytecode-version <字节码版本>     # 字节码版本
  -S, --shadow                          # 如果设置，源文件不会遮蔽依赖文件。如果同一个文件同时传递给两者，将会引发错误
      --skip-attribute-checks           # 不检查未知属性
      --debug                           # 通过打印内部信息来调试编译器
      --Wdeprecation                    # 显示关于使用已弃用函数、模块、常量等的警告。注意，此常量的当前值为 "Wdeprecation"
      --Wdeprecation-aptos              # 显示关于在 Aptos 库中使用已弃用用法的警告，我们通常不应打扰用户。注意，此常量的当前值为 "Wdeprecation-aptos"
      --Wunused                         # 显示关于未使用的函数、字段、常量等的警告。注意，此常量的当前值为 "Wunused"
      --v2                              # 支持 v2 语法（扩展阶段之前）
      --block-compiler-v1               # 阻止 v1 在扩展阶段之后运行
  -h, --help                            # 打印帮助
  -V, --version                         # 打印版本
```



```bash
move-check --help
```