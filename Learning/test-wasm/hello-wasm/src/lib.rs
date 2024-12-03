// use wasm_bindgen::prelude::*;
// use move_compiler::{Flags, shared::NumericalAddress};
// use move_compiler::diagnostics::*;
// use move_compiler::shared::*;
// use move_symbol_pool::Symbol;
// use std::collections::BTreeMap;

// // 创建编译器接口
// #[wasm_bindgen]
// pub struct MoveCompiler {
//     named_address_mapping: BTreeMap<String, NumericalAddress>,
// }

// #[wasm_bindgen]
// impl MoveCompiler {
//     #[wasm_bindgen(constructor)]
//     pub fn new() -> Self {
//         MoveCompiler {
//             named_address_mapping: BTreeMap::new(),
//         }
//     }

//     // 地址管理
//     #[wasm_bindgen]
//     pub fn add_address(&mut self, name: String, addr: String) -> Result<(), JsValue> {
//         let addr = NumericalAddress::parse_str(&addr)
//             .map_err(|e| JsValue::from_str(&format!("Invalid address: {}", e)))?;
//         self.named_address_mapping.insert(name, addr);
//         Ok(())
//     }

//     // 编译过程
//     #[wasm_bindgen]
//     pub fn compile(&self, source: &str) -> Result<JsValue, JsValue> {
//         // 1. 转换命名地址映射为编译器需要的格式
//         let named_addr_map: BTreeMap<Symbol, NumericalAddress> = self
//             .named_address_mapping
//             .iter()
//             .map(|(k, v)| (Symbol::from(k.as_str()), *v))
//             .collect();

//         // 2. 准备源文件路径
//         let target_file = Symbol::from(source);
//         let targets = vec![target_file];
        
//         // 3. 设置编译标志
//         let flags = Flags::empty().set_sources_shadow_deps(true);

//         // 4. 使用新的编译器 API
//         let compiler = move_compiler::Compiler::from_files(
//             targets,      // 目标文件
//             vec![],      // 依赖文件
//             named_addr_map,  // 地址映射
//             flags,       // 编译标志
//             &Default::default(), // 已知属性集合
//         );

//         // 5. 执行编译并处理结果
//         match compiler.build_and_report() {
//             Ok(_units) => {
//                 let result = "Compilation successful".to_string();
//                 serde_wasm_bindgen::to_value(&result)
//                     .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
//             }
//             Err(diags) => {
//                 let error_messages: Vec<String> = diags.into_iter()
//                     .map(|d| format!("{:#}", d))
//                     .collect();
//                 Err(JsValue::from_str(&error_messages.join("\n")))
//             }
//         }
//     }
// }


use move_compiler::{
  compiled_unit::AnnotatedCompiledUnit,
  diagnostics::{Diagnostics, FilesSourceText},
  parser::{self, syntax::parse_file_string},
  shared::{CompilationEnv, Flags, NumericalAddress},
  SteppedCompiler, PASS_COMPILATION,
};
use move_core_types::account_address::AccountAddress;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

// 简化的模块结构
#[derive(Serialize, Deserialize)]
pub struct SimpleModule {
  pub source: String,
  pub address: String,
}

// 简化的编译结果
#[derive(Serialize, Deserialize)]
pub struct CompileResult {
  pub success: bool,
  pub message: String,
  pub bytecode: Option<Vec<u8>>,
}

pub struct SimpleCompiler {
  source: String,
  address: String,
}

impl SimpleCompiler {
  pub fn new(module: SimpleModule) -> Self {
      Self {
          source: module.source,
          address: module.address,
      }
  }

  // 简化的编译方法
  pub fn compile(&self) -> CompileResult {
      // 创建编译环境
      let mut env = CompilationEnv::new(Flags::empty(), vec![]);
      let mut files: FilesSourceText = HashMap::new();
      let mut diags = Diagnostics::new();

      // 解析源代码
      let (defs, _) = match parse_file_string(&mut env, &self.source, &self.source) {
          Ok(result) => result,
          Err(ds) => {
              return CompileResult {
                  success: false,
                  message: "Parse error".to_string(),
                  bytecode: None,
              }
          }
      };

      // 设置地址映射
      let address = match NumericalAddress::parse_str(&self.address) {
          Ok(addr) => addr,
          Err(_) => {
              return CompileResult {
                  success: false,
                  message: "Invalid address".to_string(),
                  bytecode: None,
              }
          }
      };

      // 构建编译程序
      let program = parser::ast::Program {
          source_definitions: defs,
          lib_definitions: vec![],
          named_address_maps: Default::default(),
      };

      // 执行编译
      match SteppedCompiler::new_at_parser(env, None, program).run::<PASS_COMPILATION>() {
          Ok(compiler) => {
              let (units, _) = compiler.into_compiled_units();
              match units.first() {
                  Some(unit) => {
                      let bytecode = unit.clone().into_compiled_unit().serialize(None);
                      CompileResult {
                          success: true,
                          message: "Compilation successful".to_string(),
                          bytecode: Some(bytecode),
                      }
                  }
                  None => CompileResult {
                      success: false,
                      message: "No output generated".to_string(),
                      bytecode: None,
                  },
              }
          }
          Err(_) => CompileResult {
              success: false,
              message: "Compilation failed".to_string(),
              bytecode: None,
          },
      }
  }
}

// WebAssembly 导出接口
#[wasm_bindgen]
pub fn compile_move(module: JsValue) -> JsValue {
  let module: SimpleModule = serde_wasm_bindgen::from_value(module).unwrap();
  let compiler = SimpleCompiler::new(module);
  let result = compiler.compile();
  serde_wasm_bindgen::to_value(&result).unwrap()
}

// 使用示例
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_simple_compile() {
      let module = SimpleModule {
          source: "module test::example { fun main() { let x = 1; } }".to_string(),
          address: "0x1".to_string(),
      };
      
      let compiler = SimpleCompiler::new(module);
      let result = compiler.compile();
      assert!(result.success);
  }
}