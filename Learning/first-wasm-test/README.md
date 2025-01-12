# First-Wasm

这是一个简单的 wasm 程序，可以通过以下方式编译出 wasm 文件：

```bash
wasm-pack build --target web # 编译出完整的 wasm 包装和代码
cargo build --target wasm32-unknown-unknown # 编译出单 wasm
```

这两个命令虽然都是用来构建 WebAssembly，但有几个重要的区别：

`wasm-pack build --target web`:

1. 生成完整的 JavaScript 绑定和包装代码，使 WASM 模块更容易在 web 环境中使用
2. 自动生成 TypeScript 类型定义文件 (.d.ts)
3. 创建一个 npm 包结构，包含:
   - package.json
   - JavaScript 胶水代码
   - .wasm 文件
   - TypeScript 定义
4. 使用 wasm-bindgen 处理导出的函数，使其更容易与 JavaScript 交互

`cargo build --target wasm32-unknown-unknown`:

1. 只生成原始的 .wasm 文件，没有任何 JavaScript 绑定
2. 不生成任何额外的包装代码或类型定义
3. 输出是一个裸的 WebAssembly 模块
4. 需要手动编写 JavaScript 代码来加载和使用这个 WASM 模块
5. 更适合底层或自定义的 WASM 使用场景

使用建议：
- 如果你在开发面向 Web 的项目，建议使用 `wasm-pack build --target web`，因为它提供了更完整的工具链支持
- 如果你需要更底层的控制，或者在非 Web 环境中使用 WASM，可以使用 `cargo build --target wasm32-unknown-unknown`

# 使用

让我详细解释一下 wasm-pack 的这些命令和你展示的项目结构：

主要命令解释：

1. `wasm-pack build` - 🏗️ 构建 npm 包
   - 将你的 Rust 代码编译成 WebAssembly
   - 生成相应的 JavaScript 包装代码
   - 创建一个可以发布到 npm 的包

2. `wasm-pack pack` - 🍱 打包但不发布
   - 将编译好的 WebAssembly 模块打包成 tar 文件
   - 适合在发布前检查打包内容

3. `wasm-pack new` - 🐑 创建新项目
   - 使用模板创建一个新的 WebAssembly 项目
   - 自动设置基本的项目结构

4. `wasm-pack publish` - 🎆 发布到 npm
   - 将包发布到 npm 仓库
   - 使其他开发者可以使用你的 WebAssembly 模块

5. `wasm-pack login` - 👤 npm 用户登录
   - 添加 npm 注册表用户账户
   - 用于发布前的认证

6. `wasm-pack test` - 👩‍🔬 测试
   - 运行 WebAssembly 相关的测试

你当前的项目结构：
```
.
├── Cargo.lock      # Rust 依赖锁定文件
├── Cargo.toml      # Rust 项目配置和依赖声明
├── README.md       # 项目说明文档
└── src
    └── lib.rs      # Rust 源代码文件，将被编译为 WebAssembly
```

这是一个标准的 Rust WebAssembly 项目结构。使用 wasm-pack 编译后，会生成额外的文件夹（通常是 `pkg/`），其中包含编译好的 WebAssembly 文件和相应的 JavaScript 胶水代码。

如果你想开始构建，通常的步骤是：
1. 在 lib.rs 中编写 Rust 代码
2. 使用 `wasm-pack build` 编译项目
3. 测试生成的 WebAssembly 模块
4. 需要时发布到 npm

# 测试

运行 `wasm-pack test --chrome` 这个命令会：

编译你的 Rust 代码为 WebAssembly
在 Chrome 浏览器中运行测试
报告测试结果

如果你想在其他浏览器中测试，wasm-pack 也支持 Firefox (`--firefox`) 和无头浏览器 (`--headless`)。
