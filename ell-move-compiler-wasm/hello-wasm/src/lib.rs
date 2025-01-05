use std::collections::BTreeMap;
use std::collections::HashMap;

use move_command_line_common::files::FileHash;
use move_compiler::compiled_unit::AnnotatedCompiledUnit;
use move_compiler::diagnostics::Diagnostics;
use move_compiler::diagnostics::report_diagnostics_to_buffer;
use move_compiler::diagnostics::FilesSourceText;
use move_compiler::parser;
use move_compiler::parser::ast::PackageDefinition;
use move_compiler::shared::NamedAddressMap;
use move_compiler::shared::NamedAddressMapIndex;
use move_compiler::shared::NamedAddressMaps;
use move_compiler::shared::NumericalAddress;
use move_compiler::MatchedFileCommentMap;
use move_compiler::SteppedCompiler;
use move_compiler::PASS_COMPILATION;
use move_core_types::account_address::AccountAddress;
use move_symbol_pool::Symbol;
use wasm_bindgen::prelude::*;
use move_compiler::parser::syntax::parse_file_string;
use move_compiler::shared::CompilationEnv;
use move_compiler::shared::Flags;
use move_compiler::shared::known_attributes::KnownAttribute;
use move_command_line_common::files;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct MoveOption<T> {
    pub value: Vec<T>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct PackageMetadata {
    pub name: String,
    pub upgrade_policy: UpgradePolicy,
    pub upgrade_number: u64,
    pub source_digest: String,
    #[serde(with = "serde_bytes")]
    pub manifest: Vec<u8>,
    pub modules: Vec<ModuleMetadata>,
    pub deps: Vec<PackageDep>,
    pub extension: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModuleMetadata {
    pub name: String,
    #[serde(with = "serde_bytes")]
    pub source: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub source_map: Vec<u8>,
    pub extension: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
pub struct PackageDep {
    pub account: AccountAddress,
    pub package_name: String,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpgradePolicy {
    pub policy: u8,
}

impl UpgradePolicy {
    pub fn arbitrary() -> Self {
        UpgradePolicy { policy: 0 }
    }

    pub fn compat() -> Self {
        UpgradePolicy { policy: 1 }
    }

    pub fn immutable() -> Self {
        UpgradePolicy { policy: 2 }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Module {
    pub package_name: String,
    pub target_symbols: Vec<Symbol>,
    pub target_source: Vec<String>,
    pub target_named_address_symbol: Vec<Symbol>,
    pub target_named_address: Vec<String>,
    pub deps_symbols: Vec<Vec<Symbol>>,
    pub deps_source: Vec<Vec<String>>
}

pub struct IndexedPackage {
    pub source: HashMap<Symbol, String>,
    pub named_address_map: NamedAddressMapIndex
}

// 编译器结构体，用于管理编译过程。
pub struct Compiler {
    package_name: String, // 包名称，标识编译的模块或库的名称。
    maps: NamedAddressMaps, // 命名地址映射，存储命名地址与实际地址之间的映射关系。
    targets: IndexedPackage, // 索引包，包含源代码和编译目标的信息。
    deps: Vec<IndexedPackage> // 赖的索引包列表，包含所有依赖的库或模块。
}

impl Compiler {
    // 检查构建模块是否成功，返回结果或错误信息。
    pub fn check_build(&self) -> Result<String, ()> {
        // 创建编译环境和诊断信息。
        let mut env = CompilationEnv::new(
            Flags::empty() , KnownAttribute::get_all_attribute_names().to_owned());
        let mut diags = Diagnostics::new();
        let mut source_definitions = Vec::new();
        let mut files: FilesSourceText = HashMap::new();
        // 遍历源文件，解析并收集定义。
        self.targets.source.iter().for_each(|(k, v)|{
            let (defs, _) = match parse_file_string(&mut env,files::FileHash::new(&v),&v) {
                Ok(defs_and_comments) => defs_and_comments,
                Err(ds) => {
                    diags.extend(ds);
                    (vec![], MatchedFileCommentMap::new())
                },
            };
            source_definitions.extend(defs.into_iter().map(|def| PackageDefinition {
                package: None,
                named_address_map: self.targets.named_address_map,
                def,
            }));
            files.insert(FileHash::new(&v), (k.clone(), v.clone()));
        });

        // 遍历依赖包，解析并收集定义。
        let mut lib_definitions = Vec::new();
        self.deps.iter().for_each(|package|{
            package.source.iter().for_each(|(k, v)|{
                let (defs, _) = match parse_file_string(&mut env,files::FileHash::new(v.as_str()),v.as_str()) {
                    Ok(defs_and_comments) => defs_and_comments,
                    Err(ds) => {
                        diags.extend(ds);
                        (vec![], MatchedFileCommentMap::new())
                    },
                };
                lib_definitions.extend(defs.into_iter().map(|def| PackageDefinition {
                    package: None,
                    named_address_map: package.named_address_map,
                    def,
                }));
                files.insert(FileHash::new(&v), (k.clone(), v.clone()));
            });
        });

        // 如果没有诊断信息，则尝试编译。
        let commts: BTreeMap<u32, String> = BTreeMap::new();
        if diags.is_empty(){
            let pprog = parser::ast::Program {
                named_address_maps: self.maps.clone(),
                source_definitions,
                lib_definitions
            };
            let res: Result<(Vec<AnnotatedCompiledUnit>, Diagnostics), Diagnostics>  = SteppedCompiler::new_at_parser(env, None, pprog)
            .run::<PASS_COMPILATION>().map(|compiler| (commts, compiler)).map(|(_comments, stepped)| stepped.into_compiled_units());
            match res {
                Ok( (_, diags) ) => Ok(format!("{}",String::from_utf8(report_diagnostics_to_buffer( &files, diags)).unwrap()).to_string()),
                Err(diags) => Ok(format!("{}",String::from_utf8(report_diagnostics_to_buffer( &files, diags)).unwrap()).to_string())
            }
        }else {
            Ok(String::from_utf8(report_diagnostics_to_buffer( &files, diags)).unwrap())
        }
    }

    // 构建模块，返回编译后的二进制数据或错误信息。
    pub fn build(&self) -> Result<(Vec<u8>,Vec<Vec<u8>>), String>{
        // 建编译环境和诊断信息。
        let mut env = CompilationEnv::new(
            Flags::empty() , KnownAttribute::get_all_attribute_names().to_owned());
        let mut diags = Diagnostics::new();
        let mut source_definitions = Vec::new();
        let mut files: FilesSourceText = HashMap::new();
        // 遍历源文件，解析并收集定义。
        self.targets.source.iter().for_each(|(k, v)|{
            let (defs, _) = match parse_file_string(&mut env,files::FileHash::new(&v),&v) {
                Ok(defs_and_comments) => defs_and_comments,
                Err(ds) => {
                    diags.extend(ds);
                    (vec![], MatchedFileCommentMap::new())
                },
            };
            source_definitions.extend(defs.into_iter().map(|def| PackageDefinition {
                package: None,
                named_address_map: self.targets.named_address_map,
                def,
            }));
            files.insert(FileHash::new(&v), (k.clone(), v.clone()));
        });

        // 遍历依赖包，解析并收集定义。
        let mut lib_definitions = Vec::new();
        self.deps.iter().for_each(|package|{
            package.source.iter().for_each(|(k, v)|{
                let (defs, _) = match parse_file_string(&mut env,files::FileHash::new(v.as_str()),v.as_str()) {
                    Ok(defs_and_comments) => defs_and_comments,
                    Err(ds) => {
                        diags.extend(ds);
                        (vec![], MatchedFileCommentMap::new())
                    },
                };
                lib_definitions.extend(defs.into_iter().map(|def| PackageDefinition {
                    package: None,
                    named_address_map: package.named_address_map,
                    def,
                }));
                files.insert(FileHash::new(&v), (k.clone(), v.clone()));
            });
        });

        // 如果没有诊断信息，则尝试编译。
        let commts: BTreeMap<u32, String> = BTreeMap::new();
        if diags.is_empty(){
            let pprog = parser::ast::Program {
                named_address_maps: self.maps.clone(),
                source_definitions,
                lib_definitions
            };
            let res: Result<(_, SteppedCompiler<PASS_COMPILATION>), Diagnostics>  = SteppedCompiler::new_at_parser(env, None, pprog)
            .run::<PASS_COMPILATION>().map(|compiler| (commts, compiler));
            match res {
                Ok( ( _, stepped) ) => {
                    let units =  stepped.into_compiled_units().0;
                    Ok((bcs::to_bytes(
                    &PackageMetadata{ 
                        name: self.package_name.clone(), 
                        upgrade_policy: UpgradePolicy::compat(), 
                        upgrade_number: 0, 
                        source_digest: "".into(), 
                        manifest: vec![], 
                        modules: units.iter().map(|unit| ModuleMetadata{
                            name: unit.clone().into_compiled_unit().name().as_str().to_string(),
                            source: vec![], 
                            source_map: vec![], 
                            extension: vec![], 
                        }).collect(), 
                        deps: vec![PackageDep{ account: AccountAddress::ONE, package_name: "MoveStdlib".into() }], 
                        extension: vec![]
                    }).unwrap(), units.into_iter().map(|unit|unit.into_compiled_unit().serialize(Some(6))).collect()))
                },
                Err(diags) => Err(format!("{}",String::from_utf8(report_diagnostics_to_buffer( &files, diags)).unwrap()).to_string())
            }
        }else {
            Err(String::from_utf8(report_diagnostics_to_buffer( &files, diags)).unwrap())
        }
    }
}

impl Module {
    pub fn to_indexed_package(self)->Compiler {
        let Module { 
            package_name,
            target_named_address,
            target_named_address_symbol,
            target_source,
            target_symbols,
            deps_source,
            deps_symbols
        } = self;

        let target_named_address = target_named_address_symbol.into_iter().zip(target_named_address.into_iter()).collect::<BTreeMap<Symbol, String>>();

        let mut maps = NamedAddressMaps::new();

        let targets_idx = maps.insert(
            target_named_address
            .into_iter()
            .map(|(k,v)|(k, NumericalAddress::parse_str(v.as_str()).unwrap()))
            .collect::<NamedAddressMap>()
        );

        let targets = target_symbols.into_iter().zip(target_source.into_iter()).collect::<HashMap<Symbol, String>>();
        let deps = deps_symbols.into_iter().zip(deps_source.into_iter()).collect::<Vec<_>>();
        let deps_source = deps.into_iter().map(|(k, v)|{
            k.into_iter().zip(v.into_iter()).collect::<HashMap<Symbol, String>>()
        }).collect::<Vec<_>>();
        Compiler {
            package_name,
            maps,
            targets: IndexedPackage {
                source: targets,
                named_address_map: targets_idx
            },
            deps: deps_source.iter().map(
                | source | (
                    IndexedPackage {
                        source: source.clone().iter().map(|(k, v)|(k.to_owned().into(), v.to_owned())).collect(),
                        named_address_map: targets_idx
                    }
                )
            ).collect()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CheckBuildResponse {
    pub response: String
} 

#[derive(Serialize, Deserialize)]
pub struct BuildResponse {
    pub metadata: Vec<u8>,
    pub units: Vec<Vec<u8>>,
    pub response: String,
}

// #[wasm_bindgen]：这个属性宏标记函数，使其可以被 JavaScript 调用。
// 函数接受一个 JsValue 类型的参数，并返回一个 JsValue 类型的结果。
#[wasm_bindgen]
pub fn check_build_module(_module: JsValue) -> JsValue {
   // 将 JavaScript 传递的 JsValue 反序列化为 Rust 中的 Module 类型。
    let module: Module = serde_wasm_bindgen::from_value(_module).unwrap();
    // 将 Module 转换为编译器可以处理的索引包格式。
    let compiler = module.to_indexed_package();
    // 调用编译器的 check_build 方法检查模块是否可以构建，期望成功。
    let str = compiler.check_build().unwrap();
    // 将 Rust 中的 CheckBuildResponse 结构体序列化为 JsValue 返回给 JavaScript。
    serde_wasm_bindgen::to_value(&CheckBuildResponse{response: str.into()}).unwrap()
}

// #[wasm_bindgen]：标记函数，使其可以被 JavaScript 调用。
// 函数接受一个 JsValue 类型的参数，并返回一个 JsValue 类型的结果。
#[wasm_bindgen]
pub fn build_module(_module: JsValue) -> JsValue {
  // 将 JavaScript 传递的 JsValue 反序列化为 Rust 中的 Module 类型。
    let module: Module = serde_wasm_bindgen::from_value(_module).unwrap();
    // 将 Module 转换为编译器可以处理的索引包格式。
    let compiler = module.to_indexed_package();
    // 尝试构建模块，返回 Result 类型，包含构建的元数据和单元。
    match compiler.build()  {
      // 如果构建成功，将结果封装在 BuildResponse 结构体中，并序列化为 JsValue 返回。
        Ok((metadata, units)) => serde_wasm_bindgen::to_value(&BuildResponse{metadata, units, response:"".into()}).unwrap(),
        // 如果构建失败，将错误信息封装在 BuildResponse 结构体中，并序列化为 JsValue 返回。
        Err(str) => serde_wasm_bindgen::to_value(&BuildResponse{metadata: vec![], units: vec![], response: str.into()}).unwrap(),
    }
}

// #[cfg(test)] 属性标记，这意味着它仅在执行 cargo test 时被编译和运行。
// 测试模块包含了两个测试函数 it_works 和 build，它们分别测试编译和构建 Move 语言模块的功能。
#[cfg(test)]
mod tests {
  // 引入所需的模块和类型。
    use std::collections::{BTreeMap, HashMap};

    use move_command_line_common::files;
    use move_compiler::{diagnostics::{report_diagnostics_to_buffer, Diagnostics, FilesSourceText}, parser::{self, ast::PackageDefinition, syntax::parse_file_string}, shared::{known_attributes::KnownAttribute, CompilationEnv, NamedAddressMaps, NumericalAddress}, Flags, MatchedFileCommentMap, SteppedCompiler, PASS_COMPILATION};
    use move_core_types::account_address::AccountAddress;
    use move_symbol_pool::Symbol;

    use crate::{PackageDep, PackageMetadata, UpgradePolicy};

    // 定义一个测试函数 `it_works`，用于测试编译过程。
    #[test]
    fn it_works() {
        // 定义要编译的 Move 代码字符串。
        let str = "module test_module::test_module { fun test() { let i = 1; i = b\"3\"; } }";
        // 创建文件源文本的 HashMap。
        let mut files: FilesSourceText = HashMap::new();
        // 计算文件内容的哈希。
        let str_hash = files::FileHash::new(str);
        // 定义文件名。
        let fname = "source.move";
        // 创建命名地址映射。
        let mut named_address_maps = NamedAddressMaps::new();
        // 添加命名地址映射条目。
        let mut named_address = BTreeMap::new();
        // 将文件内容插入 HashMap。
        named_address.insert( Symbol::from("test_module"), NumericalAddress::parse_str("0xcafe").unwrap());
        let idx = named_address_maps.insert(named_address);
        // 创建诊断信息对象。
        files.insert(str_hash,(fname.into(), str.to_string()) );
        let mut diags = Diagnostics::new();
        // 创建编译环境。
        let mut env = CompilationEnv::new(
            Flags::empty() , KnownAttribute::get_all_attribute_names().to_owned());
        // 解析文件字符串。
        let (defs, comments) = match parse_file_string(&mut env,files::FileHash::new(str),str) {
            Ok(defs_and_comments) => defs_and_comments,
            Err(ds) => {
                diags.extend(ds);
                (vec![], MatchedFileCommentMap::new())
            },
        };
        // 创建源定义向量。
        let mut source_definitions = Vec::new();
        // 将定义添加到向量。
        source_definitions.extend(defs.into_iter().map(|def| PackageDefinition {
            package: None,
            named_address_map: idx,
            def,
        }));

        // 如果没有诊断信息，则尝试编译。
        if diags.is_empty(){
            let pprog = parser::ast::Program {
                named_address_maps,
                source_definitions,
                lib_definitions: Vec::new()
            };
            // 运行编译过程。
            let res: Result<_, Diagnostics>  = SteppedCompiler::new_at_parser(env, None, pprog)
            .run::<PASS_COMPILATION>().map(|compiler| (comments, compiler));
          // 根据编译结果打印信息。
            if res.is_ok(){
                println!("{}",&format!("编译成功"))
            }else {
                let diags = res.err().unwrap();
                println!("{}",&format!("{}",String::from_utf8(report_diagnostics_to_buffer( &files, diags)).unwrap())) 
            }
        }else {
            println!("{}",&format!("{}",String::from_utf8(report_diagnostics_to_buffer( &files, diags)).unwrap())) 
        }
    }

    // 定义另一个测试函数 `build`，用于测试构建过程。
    #[test]
    fn build (){
      // 定义要编译的 Move 代码字符串。
        let str = "module test_module::test_module { fun test() { let i = 1; } }";
        // 创建文件源文本的 HashMap。
        let mut files: FilesSourceText = HashMap::new();
        // 计算文件内容的哈希。
        let str_hash = files::FileHash::new(str);
        // 定义文件名。
        let fname = "source.move";
        let mut named_address_maps = NamedAddressMaps::new();
        // 创建命名地址映射。
        let mut named_address = BTreeMap::new();
        // 添加命名地址映射条目。
        named_address.insert( Symbol::from("test_module"), NumericalAddress::parse_str("0xcafe").unwrap());
        let idx = named_address_maps.insert(named_address);
        // 将文件内容插入 HashMap。
        files.insert(str_hash,(fname.into(), str.to_string()) );
        // 创建诊断信息对象。
        let mut diags = Diagnostics::new();
        // 创建编译环境。
        let mut env = CompilationEnv::new(
            Flags::empty() , KnownAttribute::get_all_attribute_names().to_owned());
        // 解析文件字符串。
        let (defs, comments) = match parse_file_string(&mut env,files::FileHash::new(str),str) {
            Ok(defs_and_comments) => defs_and_comments,
            Err(ds) => {
                diags.extend(ds);
                (vec![], MatchedFileCommentMap::new())
            },
        };
        
        // 创建源定义向量。
        let mut source_definitions = Vec::new();
        
        // 将定义添加到向量。
        source_definitions.extend(defs.into_iter().map(|def| PackageDefinition {
            package: None,
            named_address_map: idx,
            def,
        }));

        // 如果没有诊断信息，则尝试编译。
        if diags.is_empty(){
            let pprog = parser::ast::Program {
                named_address_maps,
                source_definitions,
                lib_definitions: Vec::new()
            };
            // 运行编译过程。
            let res: Result<_, Diagnostics>  = SteppedCompiler::new_at_parser(env, None, pprog)
            .run::<PASS_COMPILATION>().map(|compiler| (comments, compiler));
          // 根据编译结果打印信息。
            if res.is_ok(){
                match res {
                  // 打印编译后的二进制数据和单元。
                    Ok( ( _, stepped) ) => println!("{:?}，{:?}",bcs::to_bytes(
                        &PackageMetadata{ 
                            name: "test".into(), 
                            upgrade_policy: UpgradePolicy::compat(), 
                            upgrade_number: 0, 
                            source_digest: "".into(), 
                            manifest: vec![], 
                            modules: vec![], 
                            deps: vec![PackageDep{ account: AccountAddress::ONE, package_name: "MoveStdlib".into() }], 
                            extension: vec![]
                        }).unwrap(), stepped.into_compiled_units().0.into_iter().map(|unit|unit.into_compiled_unit().serialize(Some(6))).collect::<Vec<Vec<u8>>>()),
                    Err(diags) => println!("{}",format!("{}",String::from_utf8(report_diagnostics_to_buffer( &files, diags)).unwrap()).to_string())
                };
            }else {
                let diags = res.err().unwrap();
                println!("{}",&format!("{}",String::from_utf8(report_diagnostics_to_buffer( &files, diags)).unwrap())) 
            }
        }else {
            println!("{}",&format!("{}",String::from_utf8(report_diagnostics_to_buffer( &files, diags)).unwrap())) 
        }
    }
}
