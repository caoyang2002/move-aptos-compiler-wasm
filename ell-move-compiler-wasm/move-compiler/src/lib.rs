// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

// 禁止代码中使用 `unsafe` 代码块，确保整个库是安全的。
#![forbid(unsafe_code)]

// 声明宏 `sp` 可以在当前 crate 中被使用，这通常用于代码生成或简化代码。
#[macro_use(sp)]
extern crate move_ir_types;

// 定义和声明一系列的子模块，每个模块负责编译器的不同部分。
pub mod attr_derivation; // 属性派生相关功能。
pub mod cfgir;          // 控制流图中间表示（CFG IR）相关功能。
pub mod command_line;   // 命令行参数解析和处理。
pub mod compiled_unit;  // 编译单元的处理。
pub mod diagnostics;    // 诊断信息的报告和管理。
pub mod expansion;      // 宏展开相关功能。
pub mod hlir;           // 高级中间表示（HLIR）相关功能。
pub mod inlining;       // 函数内联相关功能。
pub mod interface_generator; // 接口生成器，用于生成接口文件。
pub mod ir_translation; // 中间表示（IR）转换功能。
pub mod naming;         // 命名解析相关功能。
pub mod parser;         // 解析器，用于解析源代码。
pub mod shared;         // 共享工具和数据结构。
mod to_bytecode;        // 将中间表示转换为字节码的功能（非公共模块）。
pub mod typing;         // 类型检查和处理。
pub mod unit_test;      // 单元测试相关功能。
pub mod verification;   // 验证功能，用于检查代码的正确性。

// 从 `command_line` 模块中重新导出编译器相关的公共接口，使得这些接口可以直接被外部使用。
pub use command_line::{
    compiler::{
        construct_pre_compiled_lib, generate_interface_files, output_compiled_units, Compiler,
        FullyCompiledProgram, SteppedCompiler, PASS_CFGIR, PASS_COMPILATION, PASS_EXPANSION,
        PASS_HLIR, PASS_INLINING, PASS_NAMING, PASS_PARSER, PASS_TYPING,
    },
    MOVE_COMPILED_INTERFACES_DIR,
};
// 从 `parser::comments` 模块中重新导出与注释处理相关的公共接口。
pub use parser::comments::{CommentMap, FileCommentMap, MatchedFileCommentMap};
// 从 `shared` 模块中重新导出 `Flags` 类型，这个类型可能用于配置编译器的不同选项和标志。
pub use shared::Flags;
