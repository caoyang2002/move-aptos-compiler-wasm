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

pub struct Compiler {
    package_name: String, 
    maps: NamedAddressMaps,
    targets: IndexedPackage,
    deps: Vec<IndexedPackage>
}

impl Compiler {
    pub fn check_build(&self) -> Result<String, ()> {
        let mut env = CompilationEnv::new(
            Flags::empty() , KnownAttribute::get_all_attribute_names().to_owned());
        let mut diags = Diagnostics::new();
        let mut source_definitions = Vec::new();
        let mut files: FilesSourceText = HashMap::new();
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

    pub fn build(&self) -> Result<(Vec<u8>,Vec<Vec<u8>>), String>{
        let mut env = CompilationEnv::new(
            Flags::empty() , KnownAttribute::get_all_attribute_names().to_owned());
        let mut diags = Diagnostics::new();
        let mut source_definitions = Vec::new();
        let mut files: FilesSourceText = HashMap::new();
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

#[wasm_bindgen]
pub fn check_build_module(_module: JsValue) -> JsValue {
    let module: Module = serde_wasm_bindgen::from_value(_module).unwrap();
    let compiler = module.to_indexed_package();
    let str = compiler.check_build().unwrap();
    serde_wasm_bindgen::to_value(&CheckBuildResponse{response: str.into()}).unwrap()
}
#[wasm_bindgen]
pub fn build_module(_module: JsValue) -> JsValue {
    let module: Module = serde_wasm_bindgen::from_value(_module).unwrap();
    let compiler = module.to_indexed_package();
    match compiler.build()  {
        Ok((metadata, units)) => serde_wasm_bindgen::to_value(&BuildResponse{metadata, units, response:"".into()}).unwrap(),
        Err(str) => serde_wasm_bindgen::to_value(&BuildResponse{metadata: vec![], units: vec![], response: str.into()}).unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashMap};

    use move_command_line_common::files;
    use move_compiler::{diagnostics::{report_diagnostics_to_buffer, Diagnostics, FilesSourceText}, parser::{self, ast::PackageDefinition, syntax::parse_file_string}, shared::{known_attributes::KnownAttribute, CompilationEnv, NamedAddressMaps, NumericalAddress}, Flags, MatchedFileCommentMap, SteppedCompiler, PASS_COMPILATION};
    use move_core_types::account_address::AccountAddress;
    use move_symbol_pool::Symbol;

    use crate::{PackageDep, PackageMetadata, UpgradePolicy};

    #[test]
    fn it_works() {
        let str = "module test_module::test_module { fun test() { let i = 1; i = b\"3\"; } }";
        let mut files: FilesSourceText = HashMap::new();
        let str_hash = files::FileHash::new(str);
        let fname = "source.move";
        let mut named_address_maps = NamedAddressMaps::new();
        let mut named_address = BTreeMap::new();
        named_address.insert( Symbol::from("test_module"), NumericalAddress::parse_str("0xcafe").unwrap());
        let idx = named_address_maps.insert(named_address);
        files.insert(str_hash,(fname.into(), str.to_string()) );
        let mut diags = Diagnostics::new();
        let mut env = CompilationEnv::new(
            Flags::empty() , KnownAttribute::get_all_attribute_names().to_owned());
        let (defs, comments) = match parse_file_string(&mut env,files::FileHash::new(str),str) {
            Ok(defs_and_comments) => defs_and_comments,
            Err(ds) => {
                diags.extend(ds);
                (vec![], MatchedFileCommentMap::new())
            },
        };
        let mut source_definitions = Vec::new();
        source_definitions.extend(defs.into_iter().map(|def| PackageDefinition {
            package: None,
            named_address_map: idx,
            def,
        }));

        if diags.is_empty(){
            let pprog = parser::ast::Program {
                named_address_maps,
                source_definitions,
                lib_definitions: Vec::new()
            };
            let res: Result<_, Diagnostics>  = SteppedCompiler::new_at_parser(env, None, pprog)
            .run::<PASS_COMPILATION>().map(|compiler| (comments, compiler));
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

    #[test]
    fn build (){
        let str = "module test_module::test_module { fun test() { let i = 1; } }";
        let mut files: FilesSourceText = HashMap::new();
        let str_hash = files::FileHash::new(str);
        let fname = "source.move";
        let mut named_address_maps = NamedAddressMaps::new();
        let mut named_address = BTreeMap::new();
        named_address.insert( Symbol::from("test_module"), NumericalAddress::parse_str("0xcafe").unwrap());
        let idx = named_address_maps.insert(named_address);
        files.insert(str_hash,(fname.into(), str.to_string()) );
        let mut diags = Diagnostics::new();
        let mut env = CompilationEnv::new(
            Flags::empty() , KnownAttribute::get_all_attribute_names().to_owned());
        let (defs, comments) = match parse_file_string(&mut env,files::FileHash::new(str),str) {
            Ok(defs_and_comments) => defs_and_comments,
            Err(ds) => {
                diags.extend(ds);
                (vec![], MatchedFileCommentMap::new())
            },
        };
        let mut source_definitions = Vec::new();
        source_definitions.extend(defs.into_iter().map(|def| PackageDefinition {
            package: None,
            named_address_map: idx,
            def,
        }));

        if diags.is_empty(){
            let pprog = parser::ast::Program {
                named_address_maps,
                source_definitions,
                lib_definitions: Vec::new()
            };
            let res: Result<_, Diagnostics>  = SteppedCompiler::new_at_parser(env, None, pprog)
            .run::<PASS_COMPILATION>().map(|compiler| (comments, compiler));
            if res.is_ok(){
                match res {
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
