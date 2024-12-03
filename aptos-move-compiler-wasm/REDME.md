# Move wasm compiler

## 项目结构

```toml
# 工作空间配置
[workspace]
resolver = "2"               # 使用新版依赖解析器，更好地处理特性启用
members = [                  # 工作空间成员列表
    "hello-wasm",           # WASM 项目
    "move-compiler"         # Move 编译器项目
]

# 工作空间级别的包配置，会被所有成员继承
[workspace.package]
authors = ["WGB5445 <wgb98512@gmail.com>"]  # 默认作者
edition = "2021"            # Rust 2021 版本
license = "Apache-2.0"      # Apache 2.0 许可证
publish = false             # 禁止发布到 crates.io
repository = ""             # 代码仓库地址（当前为空）
rust-version = "1.75.0"     # 最低支持的 Rust 版本

# 工作空间级别的依赖声明，可被成员包引用
[workspace.dependencies]
# WebAssembly 相关
wasm-bindgen = "0.2"                # Rust 和 JS 交互
getrandom = { version = "0.2", features = ["js"] }  # 随机数生成(支持 JS)
serde-wasm-bindgen = "0.4"          # Serde WASM 支持
serde = { version = "1.0.193", features = ["derive", "rc"] }  # 序列化框架
serde_bytes = "0.11.6"              # 字节序列化

# 工具库
anyhow = "1.0.71"                   # 错误处理
clap = { version = "4.3.9", features = ["derive", "env", "unstable-styles"] }  # 命令行解析
codespan-reporting = "0.11.1"       # 代码诊断报告
hex = { version = "0.4.3", features = ["serde"] }  # 16进制编解码
once_cell = "1.10.0"                # 懒加载静态变量
petgraph = "0.5.1"                  # 图算法库
regex = "1.9.3"                     # 正则表达式
sha3 = "0.9.1"                      # SHA3 哈希
tempfile = "3.3.0"                  # 临时文件处理
datatest-stable = "0.1.1"           # 测试框架

# Aptos 相关依赖
bcs = { git = "https://github.com/aptos-labs/bcs.git", rev = "d31fab9d81748e2594be5cd5cdf845786a30562d" }
move-stdlib = { git = "https://github.com/aptos-labs/aptos-core", branch = "main" }
move-command-line-common = { git = "https://github.com/aptos-labs/aptos-core", branch = "main" }
move-symbol-pool = { git = "https://github.com/aptos-labs/aptos-core", branch = "main" }
move-core-types = { git = "https://github.com/aptos-labs/aptos-core", branch = "main" }
aptos-types = { git = "https://github.com/aptos-labs/aptos-core", branch = "main" }

# 依赖补丁，用于覆盖 crates.io 上的包版本
[patch.crates-io]
x25519-dalek = { git = "https://github.com/aptos-labs/x25519-dalek", branch = "zeroize_v1" }
```
