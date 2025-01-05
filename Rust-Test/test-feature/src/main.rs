// 目前只能在nightly版本下使用
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

// 定义一个断言枚举，用于存储编译时常量
pub enum Assert<const CHECK: bool> {}

// 定义一个 IsTrue trait，用于提供默认实现
pub trait IsTrue {}

// 为 Assert<true> 提供 IsTrue 的实现
impl IsTrue for Assert<true> {}

// 定义 something 函数，它接受一个泛型参数 T
// 其中 Assert<{ size_of::<T>() < 768 }> 必须为 IsTrue 类型
fn something<T>(val: T)
where
    Assert<{ std::mem::size_of::<T>() < 768 }>: IsTrue,
{
    // 函数体
}

fn main() {
    something([0u8; 0]); // ok
    something([0u8; 512]); // ok
    // something([0u8; 1024]); // 编译错误，数组长度超过了768字节的限制
}