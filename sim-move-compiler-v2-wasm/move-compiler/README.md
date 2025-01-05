# Move 源语言

## 概要

Move 源语言是一种符合人体工程学的语言，用于编写编译成 Move 字节码的模块和脚本。

## 概览

Move 源语言是一种基于表达式的语言，旨在简化编写 Move 程序——模块和脚本——而不隐藏 Move 字节码中的核心概念。

目前，Move 有命令行工具。

* Move Check 用于检查代码，但不生成字节码
* Move Build 用于检查然后编译成字节码

将来应该会有其他工具用于测试和实验 Move 模块。

不幸的是，目前没有语言语法或特性的文档。请参阅标准库中的示例。

## 设计原则

### 使命

提供一种极简主义、表达性强、安全且透明的语言，用于生成并与 Move 字节码链接。

### 核心原则

* **比字节码更简洁** Move 基于表达式，这使得编写简洁且结构化的程序成为可能，无需额外的局部变量或结构。Move 字节码是基于栈的（加上局部变量），因此没有栈访问权限的语言需要比字节码更冗长。在 Move 源语言中，表达式允许以受控和安全的方式直接在栈上编程，因此，该语言提供了与字节码相同的功能级别，但在更简洁和易读的环境中。

* **Move 字节码透明度** Move 源语言试图将 Move 字节码中的概念提升到源语言中；它不是试图隐藏它们。字节码已经有一些强烈的观点（比你在字节码语言中可能预期的要强得多），源语言试图保持这种编程模型和思维方式。这一原则的意图是消除直接编写字节码的需要。此外，这意味着与发布模块中声明的函数和类型完全互操作。

* **比字节码更严格** 源语言通常增加了额外的限制。在表达式级别，这意味着不允许任意操作栈（只能通过表达式进行），并且没有死代码或未使用的效果。在模块级别，这可能意味着对未使用的类型或不可调用的函数发出额外警告。在概念/程序级别，这也意味着添加集成形式验证。

### 次要原则

* **学习路径** 语法选择和错误消息旨在提供自然的学习方法。例如，围绕表达式语法的一些选择可以更改为更熟悉各种其他语言，但它们会损害基于表达式的语法的即插即用感觉，这可能会损害对 Move 源语言的深入理解。

* **辅助常见社区模式** 随着 Move 的使用越来越多，模块的常见模式可能会出现。Move 可能会添加新的语言特性，使这些模式更容易、更清晰或更安全。但是，如果违反了语言的其他关键设计目标/原则，则不会添加。

* **语义保持优化** 优化是开发者的重要工具，因为它们允许程序员以更自然的方式编写代码。然而，执行的所有优化必须是语义保持的，以防止在优化后的代码中发生灾难性的漏洞或错误。话虽如此，Move 源语言的主要目标不是产生*大量*优化的代码，但这是一个不错的特性。

### 非原则

* **重抽象** Move 源语言不打算隐藏 Move 字节码的细节，从引用到全局存储。可能会有一些抽象，使与这些项的交互更容易，但它们应该始终在 Move 的最基本（字节码等价）级别上可用。这并不意味着源语言目前提供的便利性，如易于字段访问或隐式冻结，违背了核心原则集，但便利性不应在字节码级别上的交互方式上含糊或不透明。注意，这并不妨碍向语言添加功能，如访问修饰符，这些修饰符翻译为编译器生成的动态检查。只是这不是语言积极添加重抽象的目标，仅仅为了掩盖字节码设计选择。

## 命令行选项

两个可用的程序是 Move check 和 Move build。

* 可以使用 `cargo build -p move-compiler` 构建它们
* 或直接运行
  * `cargo run -p move-compiler --bin move-check -- [ARGS]`
  * `cargo run -p move-compiler --bin move-build -- [ARGS]`

Move check 是一个命令行工具，用于检查 Move 程序而不生成字节码

```text
move-check 0.0.1
检查 Move 源代码，不编译成字节码。

USAGE:
    move-check [OPTIONS] [--] [PATH_TO_SOURCE_FILE]...

FLAGS:
    -h, --help       打印帮助信息
    -V, --version    打印版本信息

OPTIONS:
    -s, --sender <ADDRESS>                           模块和脚本的发送者地址
    -d, --dependency <PATH_TO_DEPENDENCY_FILE>...    所需的库文件作为依赖项

ARGS:
    <PATH_TO_SOURCE_FILE>...    要检查的源文件
```

Move build 是一个命令行工具，用于检查 Move 程序并生成序列化字节码。
不会编译依赖项。

```text
move-build 0.0.1
将 Move 源代码编译成 Move 字节码。

USAGE:
    move-build [FLAGS] [OPTIONS] [--] [PATH_TO_SOURCE_FILE]...

FLAGS:
    -m, --source-map    将字节码源映射保存到磁盘
    -h, --help          打印帮助信息
    -V, --version       打印版本信息

OPTIONS:
    -s, --sender <ADDRESS>                           模块和脚本的发送者地址
    -d, --dependency <PATH_TO_DEPENDENCY_FILE>...    所需的库文件作为依赖项
    -o, --out-dir <PATH_TO_OUTPUT_DIRECTORY>         Move 字节码输出目录 [default: build]

ARGS:
    <PATH_TO_SOURCE_FILE>...    要检查和编译的源文件
```

## 文件夹结构

```text
move-compiler                                 # 主 crate
├── src                                       # Move 语言的源代码
│   ├── lib.rs                                # 编译的入口点
|   |
│   ├── parser                                # 将源输入解析为 AST
│   │   ├── ast.rs                            # 解析的目标 AST
│   │   ├── mod.rs                            # 解析步骤的模块
│   │   ├── lexer.rs                          # 词法分析器
│   │   └── syntax.rs                         # 解析器
|   |
│   ├── expansion                             # 展开模块别名。修复语法中无法完全用语法表达的语法（例如赋值和打包）
│   │   ├── ast.rs                            # 展开的目标 AST
│   │   ├── mod.rs                            # 展开步骤的模块
│   │   └── translate.rs                      # 解析器 ~> 展开
|   |
│   ├── naming                                # 解析名称。包括当前模块中的名称、泛型、局部变量和内置类型/函数
│   │   ├── ast.rs                            # 命名的目标 AST
│   │   ├── mod.rs                            # 命名步骤的模块
│   │   └── translate.rs                      # 展开 ~> 命名
|   |
│   ├── typing                                # 对程序进行类型检查。检查是双向的，即在检查类型的同时推断类型
│   |   ├── ast.rs                            # 类型检查的目标 AST
│   |   ├── mod.rs                            # 类型检查步骤的模块
│   |   ├── translate.rs                      # 命名 ~> 类型检查
│   |   ├── core.rs                           # 核心类型系统代码。包括类型上下文和类型的规则
│   |   ├── expand.rs                         # 类型推断后，展开所有类型变量的推断值
│   |   └── globals.rs                        # 展开类型变量后，检查资源的正确访问（检查获取）
|   |
│   ├── hlir                                  # 高级 IR。将 AST 转换为基于语句的表示，而不是基于表达式的
│   │   ├── ast.rs                            # 语句化的目标 AST
│   │   ├── mod.rs                            # 高级 IR 步骤的模块
│   │   └── translate.rs                      # 类型检查 ~> 高级 IR
|   |
│   ├── cfgir                                 # 控制流图 IR。它移除了结构化的控制流，并将块放入 CFG 中。然后执行控制流敏感的检查
│   │   ├── ast.rs                            # CFG-化的目标 AST
│   │   ├── mod.rs                            # CFG IR 步骤的模块
│   │   ├── translate.rs                      # 高级 IR ~> CFG IR
│   │   ├── absint.rs                         # 控制流敏感检查的抽象解释库
│   │   ├── cfg.rs                            # 定义 CFG 本身（AST 只是标记块）
│   │   ├── locals                            # 检查正确的局部变量使用（无移动后的使用，局部变量中不留资源）
│   │       ├── mod.rs                        # 检查的模块。包括传输函数
│   │       └── state.rs                      # 抽象解释使用的状态
│   │   └── borrows                           # 借用检查器。检查引用安全性属性
│   │       ├── borrow_map.rs                 # 借用图用于抽象状态。维护内部关系，
关于引用从哪里借用
│   │       ├── mod.rs                        # 检查的模块。包括传输函数
│   │       └── state.rs                      # 抽象解释使用的状态
|   |
│   ├── to_bytecode                           # 编译成 Move 字节码。move-check 不使用
│   │   ├── mod.rs                            # 编译成字节码的模块
│   │   ├── translate.rs                      # CFG IR ~> Move 字节码
│   │   ├── context.rs                        # 上下文映射 IR 构造和字节码句柄/偏移量
│   │   ├── remove_fallthrough_jumps.rs       # CFG IR 块始终以跳转结束；Move 字节码块可以掉入。这优化了掉入的使用（移除不必要的跳转）
│   │   └── labels_to_offsets.rs              # 在字节码生成期间，使用 CFG IR 标签。这将标签切换为字节码偏移量
|   |
│   ├── shared                                # 共享实用程序
│   │   ├── mod.rs                            # 所有模块使用的共享实用程序代码（例如源位置代码）
│   │   └── unique_map.rs                     # 包装 BTreeMap，对重复值产生错误
|   |
│   ├── errors                                # 各种检查产生的错误
│   │   └── mod.rs                            # 错误模块
|   |
│   ├── command_line                          # 两个命令行二进制文件使用的实用程序
│   |   └── mod.rs                            # 命令行模块
|   |
│   └── bin                                   # 命令行二进制文件
│       ├── move-check.rs                     # 定义 move-check 命令行工具
│       └── move-build.rs                     # 定义 move-build 命令行工具
|
└── stdlib                                    # Move 标准库
    ├── modules                               # 核心模块
    └── transaction_scripts                   # 核心交易脚本
```



# 包的基本信息

```toml
# 包的基本信息
[package]
name = "move-compiler"        # 项目名称：Move 语言编译器
version = "0.0.1"            # 版本号（开发早期版本）
authors = ["Diem Association <opensource@diem.com>"]  # 原始作者（来自 Diem 项目）
description = "The definition of the Move source language, and its compiler"  # 项目描述
publish = false              # 禁止发布到 crates.io
edition = "2021"            # 使用 Rust 2021 版本
license = "Apache-2.0"      # Apache 2.0 许可证

# 运行时依赖
[dependencies]
# 通用工具库（从工作空间继承版本）
anyhow = { workspace = true }              # 错误处理
clap = { workspace = true, features = ["derive"] }  # 命令行参数解析
codespan-reporting = { workspace = true }  # 代码错误报告
hex = { workspace = true }                 # 16进制编解码
once_cell = { workspace = true }           # 延迟初始化
petgraph = { workspace = true }            # 图算法库
regex = { workspace = true }               # 正则表达式
sha3 = { workspace = true }                # SHA3 哈希算法
tempfile = { workspace = true }            # 临时文件处理
bcs = { workspace = true }                 # 二进制序列化

# Move 语言相关依赖（均来自 aptos-core 仓库）
move-binary-format = { git = "https://github.com/aptos-labs/aptos-core", branch = "main" }        # Move 字节码格式
move-borrow-graph = { git = "https://github.com/aptos-labs/aptos-core", branch = "main" }         # 借用检查图
move-bytecode-source-map = { git = "https://github.com/aptos-labs/aptos-core", branch = "main" }  # 字节码源码映射
move-bytecode-verifier = { git = "https://github.com/aptos-labs/aptos-core", branch = "main" }    # 字节码验证器
move-command-line-common = { git = "https://github.com/aptos-labs/aptos-core", branch = "main" }  # CLI 通用功能
move-core-types = { git = "https://github.com/aptos-labs/aptos-core", branch = "main" }           # 核心类型定义
move-ir-to-bytecode = { git = "https://github.com/aptos-labs/aptos-core", branch = "main" }       # IR 到字节码转换
move-ir-types = { git = "https://github.com/aptos-labs/aptos-core", branch = "main" }             # IR 类型定义
move-symbol-pool = { git = "https://github.com/aptos-labs/aptos-core", branch = "main" }          # 符号池

# 测试依赖
[dev-dependencies]
datatest-stable = { workspace = true }     # 测试框架
move-stdlib = { git = "https://github.com/aptos-labs/aptos-core", branch = "main" }  # Move 标准库

# 测试配置
[[test]]
name = "move_check_testsuite"    # 测试套件名称
harness = false                  # 禁用默认测试框架
```
