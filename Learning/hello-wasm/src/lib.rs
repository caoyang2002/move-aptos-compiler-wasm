extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // 导入 window.alert
    fn alert(s: &str);

    // 导入 console.log
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // 导入更丰富的 console.log，支持多参数
    #[wasm_bindgen(js_namespace = console)]
    fn log_many(a: &str, b: &str);

    // HTML 元素类型
    type HTMLElement;

    #[wasm_bindgen(method)]
    fn click(this: &HTMLElement);
}

#[wasm_bindgen]
pub fn helloworld(name: &str) -> Result<(), JsValue> {
    // 检查输入
    if name.is_empty() {
        log("错误: 输入为空");
        return Err(JsValue::from_str("名字不能为空"));
    }

    // 使用 console.log 进行调试
    log_many("调用 helloworld 函数，参数:", name);

    // 显示欢迎信息
    alert(&format!("Hello World : {}!", name));

    Ok(())
}
