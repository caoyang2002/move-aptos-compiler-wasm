

# Rust 编译为WebAssembly 在前端项目中使用

> 原文：https://cloud.tencent.com/developer/article/2351474
>
> 流程：编译 wasm 的时候会使用 js 暴露一个接口，这个接口可以被外部调用。

## 前言

最近，不是加大了对`Rust`相关文章的输出吗，在评论区或者私信区。有一些不同的声音说：“Rust没有前途，然后...."。其实呢，看一个技术是否有需要学习的动力。想必大家的底层理由都是**「一切都是向钱看」**，毕竟在国内大家都是业务为主，想自己纯手搞一套符合自己的技术框架和范式，这是不切实际的。（当然也不能一杆子打死，还是有很多技术大牛的）现在大家纠结或者对这个技术属于观望态度，无非就是在平时开发工作中没有涉及到的点。

同时，由于国内技术的**「滞后性」**，有一些应用场景其实还是处于蛮荒的状态。（不是崇洋媚外，事实确实如此）。所以，在一些可以用到新的技术点的方向上，国内还是处于蓝海阶段。

所以，本着对该技术的独有关注度，我还是选择义无反顾的投身到学习和实际中。**「冲破黎明之前的黑暗，你会拥有太阳，而晨曦中第一缕阳光也是为你而耀眼」**。

![img](https://developer.qcloudimg.com/http-save/yehe-9016259/f12c004180492de7c0fa8b0530ae44dc.png)

而具体，`Rust`到底能给你带来点啥，我们之前有[文章](https://cloud.tencent.com/developer/tools/blog-entry?target=https%3A%2F%2Fmp.weixin.qq.com%2Fs%3F__biz%3DMzg3NjU2OTE1Mw%3D%3D%26mid%3D2247489662%26idx%3D1%26sn%3De4eba6d631e7aee9838aa974861f10ec%26scene%3D21%23wechat_redirect&objectId=2351474&objectType=1)讲过，这里就不在赘述了。

`Last but not leaset`,由于现在本人暂时专注于前端领域居多，所以我更多关注`Rust`能为前端带来点啥。而说到`Rust`和前端，第一点的联想就是：`WebAssembly`。（如果，不了解何为`WebAssembly`，可以参考我们之前的文章[浏览器第四种语言-WebAssembly](https://cloud.tencent.com/developer/tools/blog-entry?target=https%3A%2F%2Fmp.weixin.qq.com%2Fs%3F__biz%3DMzg3NjU2OTE1Mw%3D%3D%26mid%3D2247488556%26idx%3D1%26sn%3D34632e2cf0420e0646ae248fa86de356%26scene%3D21%23wechat_redirect&objectId=2351474&objectType=1),里面的例子是用`Emscripten`写的）

其实，我们之前写过如何用 `C` 写 `wasm`，也写过[WebAssembly-C与JS互相操作](https://cloud.tencent.com/developer/tools/blog-entry?target=https%3A%2F%2Fmp.weixin.qq.com%2Fs%3F__biz%3DMzg3NjU2OTE1Mw%3D%3D%26mid%3D2247488690%26idx%3D1%26sn%3D959db8b7099fa5404aecbee334a28368%26scene%3D21%23wechat_redirect&objectId=2351474&objectType=1)等文章。但是，由于一些不可言喻的原因搁置了。

我们今天将使用 `Rust` 创建一个 `WebAssembly Hello World` 程序。我们将深入了解由`wasm-bindgen`生成的代码，以及它们如何共同协作来帮助我们进行开发。我们还将使用`wabt`来探索生成的`wasm`代码。这将使我们更好地理解`Rust WebAssembly`，并为我们的开发奠定良好的基础。

好了，天不早了，干点正事哇。

![img](https://developer.qcloudimg.com/http-save/yehe-9016259/21375f015ba3b3d150e37aabcb5739a7.gif)

#### **1. 前置知识点**

> ❝**「前置知识点」**，只是做一个概念的介绍，不会做深度解释。因为，这些概念在下面文章中会有出现，为了让行文更加的顺畅，所以将本该在文内的概念解释放到前面来。**「如果大家对这些概念熟悉，可以直接忽略」** 同时，由于阅读我文章的群体有很多，所以有些知识点可能**「我视之若珍宝，尔视只如草芥，弃之如敝履」**。以下知识点，请**「酌情使用」**。 ❞

### **安装Rust**

如果是你一个`Rust`萌新，我们也给你提供[Rust环境配置和入门指南](https://cloud.tencent.com/developer/tools/blog-entry?target=https%3A%2F%2Fmp.weixin.qq.com%2Fs%3F__biz%3DMzg3NjU2OTE1Mw%3D%3D%26mid%3D2247486982%26idx%3D1%26sn%3Da24da0a752e8d8ca7dac993535c4faea%26scene%3D21%23wechat_redirect&objectId=2351474&objectType=1)。

如果，想独立完成安装，可以到Rust 安装页面跟着教程安装。

在安装成功`Rust`后，它会安装一个名为`rustup`的工具，这个工具能让我们管理多个不同版本的 `Rust`。默认情况下，它会安装用于惯常 `Rust` 开发的 `stable` 版本 `Rust Release`。

`Rustup` 会安装

- `Rust` 的编译器 `rustc`
- `Rust` 的包管理工具 `cargo`
- `Rust` 的标准库 `rust-std`
- 以及一些有用的文档 `rust-docs`

因为，我本机已经安装好了`Rust`。我们可以通过`rustup --version`来查看`rustup`的版本。以下是我本机的`rustup`版本信息。下文中所有的代码，都基于该版本。

代码语言：javascript

复制

```javascript
rustup --version
rustup 1.26.0 (5af9b9484 2023-04-05)
```

------

### **安装WebAssembly二进制工具包（wabt）**

![img](https://developer.qcloudimg.com/http-save/yehe-9016259/c274ca3f23d038f4190629b1a91ffa74.png)

这些工具旨在用于开发工具链或其他系统，这些系统希望**「操作WebAssembly文件」**。与`WebAssembly`规范解释器不同（该解释器旨在尽可能简单、声明性和“规范性”），这些工具是用`C/C++`编写的，并设计成更容易集成到其他系统中。这些工具不旨在提供优化平台或更高级的编译目标；相反，它们旨在实现与规范的完全适应和遵从。

我们可以利用`brew`来在`Mac`环境下安装。

![img](https://developer.qcloudimg.com/http-save/yehe-9016259/278c9ddddd0d5ec574955a515ec0c1b2.png)

------

## **2. 项目搭建**

### **2.1 安装 wasm-bindgen**

我们可以通过 `cargo install --list` 来查看在`$HOME/.cargo/bin`位置安装过的`Rust`二进制文件。

在一些其他的教程中可以不使用`wasm-bindgen`构建`Hello World`程序，但是在本文中，我们将使用它，因为它在`Rust WebAssembly`开发中是必不可少的。



```bash
cargo install wasm-bindgen-cli
```

`Rust WebAssembly`允许我们将`WebAssembly模块`有针对性地插入到现有的`JavaScript`应用程序中，尤其是在**「性能关键的代码路径」**中。我们可以将`wasm-bindgen`视为一种工具，它通过生成用于`JavaScript`和`WebAssembly`之间高效交互的**「粘合代码」**和绑定来帮助我们实现丝滑的交互体验。



------

### **2.2 创建Rust WebAssembly项目**

巴拉拉小魔仙，念诵如下咒语，构建一个`Rust WebAssembly`项目。

代码语言：javascript

复制

```bash
cargo new hello_world --lib
```

上面的代码是使用`Cargo`工具创建一个新的`Rust`项目，项目的名称是`hello_world`，并且指定它是一个库（`--lib`）。这将创建一个包含基本项目结构的文件夹，其中包括一个`Cargo.toml`文件和一个`src`文件夹。



```bash
+-- Cargo.toml
+-- src
    +-- lib.rs
```

- `Cargo.toml`文件用于管理项目的依赖和配置
- `src`文件夹包含项目的`Rust源代码文件`
- 项目名称`hello_world`是一个示例名称，我们可以根据自己的需求为项目指定不同的名称。

------

### **2.3 修改`Cargo.toml`配置项**

使用宇宙最强IDE -`VScode`，打开`Cargo.toml`文件。我们应该会看到以下内容。

代码语言：javascript

复制

```toml
[package]
name = "hello_world"
version = "0.1.0"
authors = ["789"]
edition = "2021"

[dependencies]
```

将其修改成下面的内容

```toml
[package]
name = "hello_world"
version = "0.1.0"
authors = ["789"]
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
```

上面的大部分字段大家都能看懂，其中`lib`项的配置，这里稍微解释一下：

`crate-type = ["cdylib"]`: 这一行**「指定了生成的库的类型」**。在这里，`crate-type` 设置为`["cdylib"]`，这表示我们正在创建一个动态链接库（`C-compatible dynamic library`）。这用于编译一个供其他编程语言加载的动态库。此输出类型将在`Linux`上创建`*.so`文件，在`macOS`上创建`*.dylib`文件，在`Windows`上创建`*.dll`文件。

这种类型的库可以被其他编程语言调用，因为它们与C语言兼容。这对于与WebAssembly（Wasm）互操作性很重要，因为`Wasm`通常需要与`C语言`接口进行交互。因此，`cdylib` 表示该库是一个可供其他语言使用的`动态链接库`。

------

### **2.4 编辑lib.rs**

打开`src/lib.rs`文件。将其更改为以下内容：

```rust
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

// 导入 'window.alert'
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// 导出一个 'helloworld' 函数
#[wasm_bindgen]
pub fn helloworld(name: &str) {
    alert(&format!("Hello World : {}!", name));
}
```

我们简单解释一下核心代码:

1. `extern crate wasm_bindgen;`: 这一行声明了对`wasm_bindgen`库的依赖。`wasm_bindgen`是一个Rust库，用于构建`Wasm`模块并提供与`JavaScript`的互操作性。在 `Rust` 当中，库被称为`crates`，因为我们使用的是一个外部库，所以有 `extern`。
2. `use wasm_bindgen::prelude::*;`: 这一行导入了`wasm_bindgen`库的预导出（`prelude`）模块中的所有内容，以便在后续代码中使用。

#### **在 Rust 中调用来自 JavaScript 的外部函数**

```rust
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
```

`#[wasm_bindgen]`: 在 `#[]` 中的内容叫做 "属性"，并以某种方式改变下面的语句。`#[wasm_bindgen]`是一个**「属性标记」**，用于指定与`WebAssembly`互操作相关的特性。

`extern "C" { fn alert(s: &str); }`: 这里声明了一个**「外部函数」**`alert`，它使用`extern "C"` 指定了`C ABI`（应用二进制接口），这意味着它**「可以与C语言进行交互」**。**「这个**`**alert**`**函数没有在**`**Rust**`**中实现，而是在**`**JavaScript**`**中实现，用于在浏览器中显示警告框」**。

#### **在 JavaScript 中调用的 Rust 函数**

```rust
#[wasm_bindgen]
pub fn helloworld(name: &str) {
    alert(&format!("Hello World : {}!", name));
}
```

`#[wasm_bindgen] pub fn helloworld(name: &str)`: 这是一个`Rust`函数`helloworld`，它被标记为`wasm_bindgen`，这意味着它**「可以被**`**JavaScript**`**调用」**。这个函数接受一个**「字符串参数」**`name`，然后调用**「之前声明」**的`alert`函数，以显示带有`Hello World`消息的弹框，并在消息中包括`name`参数的内容。

### **2.5 编译代码**

在命令行中输入以下命令：

```bash
cargo build --target wasm32-unknown-unknown
```

> 

❝

如果未安装对应的库，控制台会给出提示。

![img](https://developer.qcloudimg.com/http-save/yehe-9016259/2f69738210eb08ad6e87d71dae7ce49e.png)

那我们就照猫画虎的操作一下：

```bash
rustup target add wasm32-unknown-unknown
```

❞

1. `cargo build`: 这是 `Cargo` 工具的命令，用于构建 `Rust` 项目。它会编译项目的源代码并生成可执行文件或库文件，具体取决于项目的类型。
2. `--target wasm32-unknown-unknown`: 这部分是构建的目标参数。`--target` 标志用于指定要构建的目标平台。在这里，`wasm32-unknown-unknown` 是指定了 `WebAssembly` 目标平台。这告诉 `Cargo` 生成**「适用于** `**WebAssembly**` **的二进制文件」**，而不是生成本地平台的二进制文件。

当运行这个命令后，`Cargo` 会使用 `Rust` 编译器（`Rustc`）以及与 `WebAssembly` 相关的工具链，将 `Rust` 代码编译为 `WebAssembly` 格式的二进制文件。这个生成的 `Wasm` 文件可以在浏览器中运行，或与其他支持 `WebAssembly` 的环境一起使用。

运行结果如下：

![img](https://developer.qcloudimg.com/http-save/yehe-9016259/08fcde35d883b97484dfba8ca580c7bb.png)

`cargo build --target wasm32-unknown-unknown` 命令的**「默认输出位置」**是在项目的 `target` 目录下，具体位置是：

```bash
target/wasm32-unknown-unknown/debug/
```

在这个目录下，我们会找到生成的 `WebAssembly` 文件（通常是一个 `.wasm` 文件），以及其他与编译过程相关的文件。

![img](https://developer.qcloudimg.com/http-save/yehe-9016259/d370a025e4ecc73dc7c22acc84c719fe.png)

------

### **2.6 构建Web服务器**

既然，我们通过上述的魔法，将`Rust`程序编译为了可以在浏览器环境下引用执行的格式。**「为了这口醋，我们还专门包顿饺子」**。

![img](https://developer.qcloudimg.com/http-save/yehe-9016259/58108e46595578800075778d48cfa94e.gif)

我们需要一个`Web服务器`来测试我们的`WebAssembly`程序。我们将使用`Webpack`，我们需要创建三个文件：`index.js`、`package.json`和`webpack.config.js`。

下面的代码，我们最熟悉不过了，就不解释了。

#### **index.js**



```js
// 直接引入了，刚才编译后的文件
const rust = import('./pkg/hello_world.js');

rust
  .then(m => m.helloworld('World!'))
  .catch(console.error);
```

#### **package.json**



```json
{
  "scripts": {
    "build": "webpack",
    "serve": "webpack-dev-server"
  },

  "devDependencies": {
    "@wasm-tool/wasm-pack-plugin": "0.4.2",
    "text-encoding": "^0.7.0",
    "html-webpack-plugin": "^3.2.0",
    "webpack": "^4.29.4",
    "webpack-cli": "^3.1.1",
    "webpack-dev-server": "^3.1.0"
  }
}
```

#### **webpack.config.js**



```js
const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
    entry: './index.js',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'index.js',
    },
    plugins: [
        new HtmlWebpackPlugin(),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, ".")
        }),
        // 让这个示例在不包含`TextEncoder`或`TextDecoder`的Edge浏览器中正常工作。
        new webpack.ProvidePlugin({
            TextDecoder: ['text-encoding', 'TextDecoder'],
            TextEncoder: ['text-encoding', 'TextEncoder']
        })
    ],
    mode: 'development'
};
```

安装指定的依赖。

```bash
npm install webpack --save-dev
npm install webpack-cli --save-dev
npm install webpack-dev-server --save-dev
npm install html-webpack-plugin --save-dev
npm install @wasm-tool/wasm-pack-plugin --save-dev
npm install text-encoding --save-dev
```

------

### **2.7 构建&运行程序**

使用`npm run build`构建程序。

使用`npm run serve`运行`Hello World`程序

在浏览器中打开`localhost:8080`,我们将看到一个显示 `Hello World!` 的弹窗。

![img](https://developer.qcloudimg.com/http-save/yehe-9016259/1b5264e44005333c96a165279d52d8cb.png)

到目前为止，我们已经构建了一个`wasm`并且能够和`js`实现功能交互的项目。其实，到这里已经完成了，我们这篇文章的使命。但是，在这里戛然而止，感觉缺失点啥。所以，我们继续深挖上面的项目的实现原理。

------

## **3. 原理探析**

在使用`cargo`和`wasm_bindgen`编译源代码时，会在`pkg`文件中**「自动生成」**以下文件：

- "hello_world_bg.wasm"
- "hello_world.js"
- "hello_world.d.ts"
- "package.json"

这些文件也可以通过使用以下`wasm-bindgen`命令`手动生成`：

```bahs
wasm-bindgen target/wasm32-unknown-unknown/debug/hello_world.wasm --out-dir ./pkg
```

### **浏览器调用顺序**

```bash
npx http-server
```



以下显示了当我们在浏览器中访问`localhost:8080`时发生的函数调用序列。

1. `index.js`
2. `hello_world.js` (调用 `hello_world_bg.js` )
3. `helloworld_bg.wasm`

#### **index.js**

```js
const rust = import('./pkg/hello_world.js');

rust
  .then(m => m.helloworld('World!'))
  .catch(console.error);
```

`index.js` 导入了 `hello_world.js` 并调用其中的 `helloworld` 函数。

#### **hello_world.js**

下面是`hello_world.js`的内容，在其中它调用了`helloworld_bg.wasm`

```js
import * as wasm from "./hello_world_bg.wasm";
import { __wbg_set_wasm } from "./hello_world_bg.js";
__wbg_set_wasm(wasm);
export * from "./hello_world_bg.js";
```

#### **hello_world_bg.js**

```js
// ...省去了部分代码
export function helloworld(name) {
    const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.helloworld(ptr0, len0);
}
```

`hello_world_bg.js` 文件是由 `wasm-bindgen`自动生成的，它包含了用于将`DOM`和`JavaScript`函数导入到`Rust`中的`JavaScript粘合代码`。它还在生成的`WebAssembly`函数上向`JavaScript`公开了API。 

`Rust WebAssembly`专注于将`WebAssembly`与现有的`JavaScript`应用程序集成在一起。为了实现这一目标，我们需要在`JavaScript`和`WebAssembly`函数之间**「传递不同的值、对象或结构。这并不容易，因为需要协调两个不同系统的不同对象类型」**。

> 更糟糕的是，当前`WebAssembly`仅支持**「整数」**和**「浮点数」**，不支持字符串。这意味着我们不能简单地将字符串传递给`WebAssembly`函数。 

要将`字符串`传递给`WebAssembly`，我们需要**「将字符串转换为数字」**（请注意在`webpack.config.js`中指定的`TextEncoderAPI`），将这些数字放入`WebAssembly`的内存空间中，最后**「返回一个指向字符串的指针」**给`WebAssembly`函数，以便在`JavaScript`中使用它。在最后，我们需要释放`WebAssembly`使用的字符串内存空间。

如果我们查看上面的`JavaScript`代码，这正是自动执行的操作。`helloworld`函数首先调用`passStringToWasm`。

- 这个函数在`WebAssembly`中**「创建一些内存空间」**，将我们的字符串转换为数字，将数字写入内存空间，并返回一个指向字符串的指针。

![img](https://developer.qcloudimg.com/http-save/yehe-9016259/abe0e92b35ea103957ad75495c686094.png)

- 然后将指针传递给

  ```
  wasm.helloworld
  ```

  来执行

  ```
  JavaScript
  ```

  的

  ```
  alert
  ```

  。最后，

  ```
  wasm.__wbindgen_free
  ```

  释放了内存。

  1. 

  ![img](https://developer.qcloudimg.com/http-save/yehe-9016259/455b4e7c12dc4504de558a7ec8ac1057.png)

  1. 

  ![img](https://developer.qcloudimg.com/http-save/yehe-9016259/ef1290dad22cb643928f0e640624daea.png)

  1. 

  ![img](https://developer.qcloudimg.com/http-save/yehe-9016259/bba40b3e10a8707ac05f48bfd2b50f2f.png)

  1. 

  ![img](https://developer.qcloudimg.com/http-save/yehe-9016259/2b9de7e3875dc3b82e854464093ea097.png)

如果只是传递一个简单的字符串，我们可能可以自己处理，但考虑到当涉及到更复杂的对象和结构时，这个工作会很快变得非常复杂。这说明了`wasm-bindgen`在`Rust WebAssembly`开发中的重要性。

### **反编译wasm到txt**

在前面的步骤中，我们注意到`wasm-bindgen`生成了一个`hello_world.js`文件，其中的函数调用到我们生成的`hello_world_bg.wasm`中的`WebAssembly`代码。

> ❝基本上，`hello_world.js`充当其他`JavaScript`（如`index.js`）与生成的`WebAssembly`的`helloworld_bg.wasm`之间的桥梁。 ❞

我们可以通过输入以下命令进一步探索`helloworld_bg.wasm`：

代码语言：javascript

复制

```javascript
wasm2wat hello_world_bg.wasm > hello_world.txt
```

这个命令使用`wabt`将`WebAssembly`转换为`WebAssembly文本格式`，并将其保存到一个`hello_world.txt`文件中。打开`helloworld.txt`文件，然后查找`$helloworld`函数。这是我们在`src/lib.rs`中定义的`helloworld`函数的生成`WebAssembly`函数。

#### **$helloworld函数**

![img](https://developer.qcloudimg.com/http-save/yehe-9016259/975d86d734d108d3b9a8aeddd45b549f.png)

在`helloworld.txt`中查找以下行：

代码语言：javascript

复制

```javascript
(export "helloworld" (func $helloworld))
```

这一行导出了`wasm.helloworld`供宿主调用的`WebAssembly`函数。我们通过`hello_world_bg.js`中的`wasm.helloworld`来调用这个`WebAssembly`函数。

![img](https://developer.qcloudimg.com/http-save/yehe-9016259/80f55aeb6cd2e85b382e0e28239eec3a.png)

接下来，查找以下行：

代码语言：javascript

复制

```javascript
(import "./hello_world_bg.js" "__wbg_alert_9ea5a791b0d4c7a3" (func $hello_world::alert::__wbg_alert_9ea5a791b0d4c7a3::h93c656ecd0e94e40 (type 4)))
```

这对应于在`hello_world_bg.js`中生成的以下`JavaScript`函数：

代码语言：javascript

复制

```javascript
export function __wbg_alert_9ea5a791b0d4c7a3() { return logError(function (arg0, arg1) {
    alert(getStringFromWasm0(arg0, arg1));
}, arguments) };
```

这是`wasm-bindgen`提供的**「粘合部分」**，帮助我们在`WebAssembly`中使用`JavaScript`函数或`DOM`。

最后，让我们看看`wasm-bindgen`生成的其他文件。

#### **hello_world.d.ts**

这个`.d.ts`文件包含`JavaScript`粘合的`TypeScript`类型声明，如果我们的现有`JavaScript`应用程序正在使用`TypeScript`，它会很有用。我们可以对调用`WebAssembly`函数进行**「类型检查」**，或者让我们的IDE提供自动完成。如果我们不使用`TypeScript`，可以安全地忽略这个文件。

#### **package.json**

`package.json`文件包含有关生成的`JavaScript`和`WebAssembly`包的元数据。它会自动从我们的`Rust`代码中填充所有npm依赖项，并使我们能够发布到`npm`。

------

## **4. 内容拓展**

再次看一下以下代码：

### **hello_world_bg.js**

代码语言：javascript

复制

```javascript
function helloworld(name) {
    const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.helloworld(ptr0, len0);
}
```

该代码用于分配和释放内存，这一切都是**「由程序自动处理」**的。不需要垃圾回收器或完整的框架引擎，使得使用`Rust`编写的`WebAssembly`应用程序或模块变得小巧且优化。其他需要垃圾回收器的语言将需要包含用于其底层框架引擎的`wasm`代码。因此，无论它们有多么优化，其大小都不会小于`Rust`提供的大小。这使得`Rust WebAssembly`成为一个不错的选择，如果我们需要将小型`WebAssembly`模块集成或注入到`JavaScript Web`应用程序中。

除了`Hello World`之外，还有一些其他需要注意的事项：

### **web-sys**

使用`wasm-bindgen`，我们可以通过使用`extern`在`Rust WebAssembly`中调用`JavaScript`函数。请记住`src/lib.rs`中的以下代码：

代码语言：javascript

复制

```javascript
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
```

`Web`具有大量API，从`DOM`操作到`WebGL`再到`Web Audio`等等。因此，如果我们的`Rust WebAssembly`程序增长，并且我们需要对`Web API`进行多次不同的调用，我们将需要花时间编写大量的`extern`代码。

> ❝`web-sys`充当`wasm-bindgen`的前端，为所有Web API提供原始绑定。 ❞

这意味着如果我们使用`web-sys`，可以节省时间，而不必编写`extern`代码。

![img](https://developer.qcloudimg.com/http-save/yehe-9016259/57861d1510b052c47f2f30bcc3ab3924.png)

#### **引入web-sys**

将`web-sys`添加为`Cargo.toml`的依赖项：

代码语言：javascript

复制

```javascript
[dependencies]
wasm-bindgen = "0.2"

[dependencies.web-sys]
version = "0.3"
features = [
]
```

为了保持构建速度非常快，`web-sys`将每个Web接口都封装在一个`Cargo`特性后面。在API文档中找到我们要使用的类型或方法；它将列出必须启用的特性才能访问该API。

例如，如果我们要查找`window.resizeTo`函数，我们会在API文档中搜索`resizeTo`。我们将找到`web_sys::Window::resize_to`函数，它需要启用`Window`特性。要访问该函数，我们在`Cargo.toml`中启用`Window`特性：

代码语言：javascript

复制

```javascript
[dependencies.web-sys]
version = "0.3"
features = [
  "Window"
]
```

调用这个方法：

代码语言：javascript

复制

```javascript
use wasm_bindgen::prelude::*;
use web_sys::Window;

#[wasm_bindgen]
pub fn make_the_window_small() {
    // 调整窗口大小为500px x 500px。
    let window = web_sys::window().unwrap();
    window.resize_to(500, 500)
        .expect("无法调整窗口大小");
}
```

这段代码的目的是调整浏览器窗口的大小为`500x500`像素，并演示了如何使用`web-sys`和启用的`Cargo`特性来调用Web API。

本文参与 [腾讯云自媒体同步曝光计划](https://cloud.tencent.com/developer/support-plan)，分享自微信公众号。

原始发表：2023-10-23，如有侵权请联系 [cloudcommunity@tencent.com](mailto:cloudcommunity@tencent.com) 删除





# 将 Rust 代码编译为 WASM

https://www.cnblogs.com/guojikun/p/18358337



# 前言

在现代 Web 开发中，WebAssembly (WASM) 已成为一种强大的工具。它使得开发者可以在浏览器中运行高性能的代码，跨越传统的 JavaScript 性能限制。Rust 语言因其高效性和内存安全性，成为了编写 WASM 模块的热门选择。本文将介绍如何将 Rust 代码编译为 WebAssembly，并在 Web 项目中使用。

## 1. 创建 Rust 项目

首先，我们需要创建一个新的 Rust 项目。由于我们要生成一个可以被其它语言或工具调用的模块，因此选择创建一个库项目，而不是可执行程序。使用 `cargo` 命令可以轻松完成：

```bash
cargo new lib_wasm --lib
```

这个命令会生成一个名为 `lib_wasm` 的项目，其中包含一个基础的 `Cargo.toml` 配置文件和一个 `src/lib.rs` 文件，你将在其中编写你的 Rust 代码。

## 2. 添加 `wasm-bindgen` 依赖项

在 Rust 中，`wasm-bindgen` 是一个关键工具，它使 Rust 和 JavaScript 之间的交互变得更加简单。`wasm-bindgen` 负责生成与 JavaScript 交互所需的绑定代码，让你能够直接调用 Rust 编写的函数。

要添加 `wasm-bindgen`，你可以使用 `cargo add` 命令：

```bash
cargo add wasm-bindgen
```

或者，手动编辑 `Cargo.toml` 文件，添加如下依赖项：

```toml
[dependencies]
wasm-bindgen = "0.2"
```

添加 `wasm-bindgen` 后，Rust 编译器会在编译过程中生成必要的绑定文件，从而使你的 WASM 模块可以被 JavaScript 直接调用。

## 3. 安装 `wasm32-unknown-unknown` 目标

Rust 编译器默认会生成适用于本地机器架构的可执行文件。要编译成适用于 Web 的 WebAssembly 文件，我们需要添加一个特定的目标架构，即 `wasm32-unknown-unknown`。这是一个通用的 WASM 目标，不依赖任何特定的操作系统。

使用以下命令安装该目标：

```bash
rustup target add wasm32-unknown-unknown
```

此命令会配置你的 Rust 工具链，使其能够生成适用于 WebAssembly 的二进制文件。

## 4. 编写 Rust 代码

现在，你可以在 `src/lib.rs` 文件中编写你希望导出的功能。例如，我们可以编写一个简单的函数，它接受一个名字作为参数并返回一个问候语：

```rust
use wasm_bindgen::prelude::*;

// 使用 #[wasm_bindgen] 宏来导出函数到 JavaScript
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

在这段代码中，我们使用了 `#[wasm_bindgen]` 宏将 `greet` 函数导出，使其可以从 JavaScript 中调用。

## 5. 编译 Rust 项目为 WASM

编写完代码后，我们可以将其编译为 WASM 文件。编译时指定目标为 `wasm32-unknown-unknown`，并使用 `--release` 选项生成优化后的构建：

```bash
cargo build --target wasm32-unknown-unknown --release
```

编译完成后，生成的 `.wasm` 文件将存储在 `target/wasm32-unknown-unknown/release/` 目录下。

## 6. 使用 `wasm-bindgen` 生成 JavaScript 绑定代码

虽然编译生成了 `.wasm` 文件，但直接在 JavaScript 中使用它并不方便。为此，我们需要使用 `wasm-bindgen` 工具生成相应的 JavaScript 绑定代码。这将创建一个便于在 JavaScript 中调用的模块。

首先，确保已安装 `wasm-bindgen-cli` 工具。你可以通过以下命令安装：

```bash
cargo install wasm-bindgen-cli
```

然后，运行以下命令生成 JavaScript 绑定文件：

```bash
wasm-bindgen --out-dir ./out --target web target/wasm32-unknown-unknown/release/lib_wasm.wasm
```

这会在 `out` 目录中生成一系列文件，包括 `.js` 文件和 `.wasm` 文件，你可以直接在 Web 项目中使用。

## 7. 在网页中使用 WASM 模块

现在，生成的 WASM 模块已经可以在 Web 项目中使用。你只需在 HTML 文件中导入生成的 JavaScript 绑定文件，并调用 Rust 导出的函数。例如：

```html
<!DOCTYPE html>
<html>
<head>
    <title>Lib WASM Demo</title>
</head>
<body>
    <script type="module">
        import init, { greet } from "./out/lib_wasm.js";
        init().then(() => {
            console.log(greet("World"));
        });
    </script>
</body>
</html>
```

这个示例会在控制台打印出 "Hello, World!"。其中，`init` 函数用于初始化 WASM 模块，而 `greet` 函数则调用了我们在 Rust 中定义的函数。

web 项目目录结构如下：

```plaintext
index.html
out/
    ├── lib_wasm_bg.wasm
    ├── lib_wasm_bg.wasm.d.ts
    ├── lib_wasm.d.ts
    └── lib_wasm.js
```

`out` 目录中包含了生成的 WASM 文件以及相应的 JavaScript 绑定文件，`index.html` 是一个简单的网页，用于测试和展示你的 WASM 模块。

## 结语

通过这套流程，你可以轻松地将 Rust 代码编译为 WebAssembly，并将其集成到 Web 项目中。Rust 的高效性和 WebAssembly 的灵活性相结合，可以为 Web 应用带来显著的性能提升。

本文来自博客园，作者：[_zhiqiu](https://www.cnblogs.com/guojikun/)，转载请注明原文链接：https://www.cnblogs.com/guojikun/p/18358337

