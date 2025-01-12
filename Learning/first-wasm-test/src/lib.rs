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

// 添加测试
#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn pass() {
        assert_eq!(1 + 1, 2);
    }
}
