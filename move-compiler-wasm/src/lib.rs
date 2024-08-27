use wasm_bindgen::prelude::*;
use move_compiler::*;
use std::collections::BTreeMap;

// use move_compiler::Flags; // 确保这里正确导入了 Flags

use move_compiler::{
    compiled_unit::AnnotatedCompiledUnit,
    diagnostics::*,
    shared::{known_attributes::KnownAttribute, Flags, NumericalAddress},
    unit_test, CommentMap, Compiler, SteppedCompiler, PASS_CFGIR, PASS_PARSER,
};



#[wasm_bindgen]
pub fn compile_move(source: &str) -> String {
    // 设置编译选项
    let mut compiler_options = move_compiler::Flags::empty();
    compiler_options.set(Flags::SOURCES, true);
    compiler_options.set(Flags::DEPENDENCIES, true);

    // 创建一个虚拟文件系统来存储源代码
    let mut files = BTreeMap::new();
    files.insert("input.move".to_string(), source.to_string());

    // 设置编译环境
    let env = move_compiler::Compiler::new(&files, vec![])
        .set_flags(compiler_options)
        .set_named_address_values(BTreeMap::new())
        .build()
        .unwrap();

    // 执行编译
    match move_compiler::Compiler::compile(&env) {
        Ok((files, units)) => {
            // 编译成功，返回编译结果
            let mut result = String::new();
            for (file_name, file_content) in files {
                result.push_str(&format!("File: {}\n{}\n\n", file_name, file_content));
            }
            for unit in units {
                result.push_str(&format!("Module: {}\n", unit.name()));
            }
            result
        }
        Err(errors) => {
            // 编译失败，返回错误信息
            format!("Compilation errors:\n{:?}", errors)
        }
    }
}
