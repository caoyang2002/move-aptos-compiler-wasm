[一个简单的 Wasm 项目](./simple-wasm/)

[一个测试的 Move Wasm 项目](./test-wasm/)

如何将 Move 语言编译器打包成 WebAssembly (WASM) 供前端使用

```rust
// hello-wasm/src/lib.rs
use wasm_bindgen::prelude::*;
use move_compiler::shared::NumericalAddress;
use move_compiler::Flags;
use std::collections::BTreeMap;

#[wasm_bindgen]
pub struct CompilerConfig {
    named_address_mapping: BTreeMap<String, NumericalAddress>,
}

#[wasm_bindgen]
impl CompilerConfig {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        CompilerConfig {
            named_address_mapping: BTreeMap::new(),
        }
    }

    #[wasm_bindgen]
    pub fn add_address_mapping(&mut self, name: String, addr: String) {
        if let Ok(addr) = NumericalAddress::parse_str(&addr) {
            self.named_address_mapping.insert(name, addr);
        }
    }
}

#[wasm_bindgen]
pub fn compile_move(source: &str, config: &CompilerConfig) -> Result<String, JsValue> {
    let files = vec![("source.move", source)];
    
    let flags = Flags::empty().set_sources_shadow_deps(true);
    
    match move_compiler::construct_pre_compiled_lib(
        files,
        None,
        config.named_address_mapping.clone(),
        flags,
        None,
    ) {
        Ok(compiled) => Ok(format!("{:?}", compiled)),
        Err(err) => Err(JsValue::from_str(&format!("Compilation error: {:?}", err))),
    }
}

```

```toml
# hello-wasm/Cargo.toml
[package]
name = "hello-wasm"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = { workspace = true }
move-compiler = { path = "../move-compiler" }
getrandom = { workspace = true }

```

接下来是构建和使用步骤：

1. 首先确保安装了必要的工具：

```bash
# 安装 wasm-pack
cargo install wasm-pack

# 构建 wasm 包
cd hello-wasm
wasm-pack build --target web
```

2. 在前端项目中使用：

```javascript
// index.js
import init, { CompilerConfig, compile_move } from './pkg/hello_wasm.js';

async function compileMoveCode() {
    await init();
    
    const config = new CompilerConfig();
    config.add_address_mapping("std", "0x1");
    
    const moveCode = `
    module 0x1::example {
        public fun hello(): u64 { 42 }
    }
    `;
    
    try {
        const result = compile_move(moveCode, config);
        console.log("Compilation result:", result);
    } catch (error) {
        console.error("Compilation failed:", error);
    }
}

compileMoveCode();

```

主要步骤说明：

1. 项目结构：
   - 创建一个 workspace 包含两个项目：move-compiler 和 hello-wasm
   - hello-wasm 项目使用 wasm-bindgen 将 Move 编译器功能暴露给 JavaScript

2. 关键点：
   - 使用 `wasm-bindgen` 标注需要暴露给 JavaScript 的函数和类型
   - 将 crate-type 设置为 "cdylib" 以生成 WebAssembly 模块
   - 处理好内存管理和错误处理

3. 注意事项：
   - 需要确保所有依赖都支持 wasm32-unknown-unknown 目标
   - 某些系统级 API 在 WASM 环境中可能不可用，需要适当处理
   - 考虑使用 web-sys 和 js-sys 处理浏览器环境特定功能

4. 编译和部署：
   - 使用 wasm-pack 构建项目
   - 在前端项目中正确导入和初始化 WASM 模块
