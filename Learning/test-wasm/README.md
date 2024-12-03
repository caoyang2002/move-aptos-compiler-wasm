编译为 wasm 的两种方法

1. 使用 wasm-pack（推荐）:

```bash
# 先安装 wasm-pack
cargo install wasm-pack

# 然后在 hello-wasm 目录下运行构建命令
cd hello-wasm
wasm-pack build --target web
```

2. 或者直接使用 cargo:

```bash
# 先安装 wasm32 目标
rustup target add wasm32-unknown-unknown

# 在 hello-wasm 目录下编译
cd hello-wasm
cargo build --target wasm32-unknown-unknown --release
```

使用 wasm-pack 的优势是它会：

自动生成 JavaScript 包装代码
创建 npm 包结构
生成 TypeScript 类型定义
优化 wasm 文件大小

编译完成后：

如果用 wasm-pack，输出在 hello-wasm/pkg 目录
如果用 cargo，输出在 hello-wasm/target/wasm32-unknown-unknown/release