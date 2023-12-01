// #![allow(unused_variables)]
// #![allow(dead_code)]

// Rust 中的宏
// 1. 声明宏 (declarative macro)
// 2. 过程宏 (procedure macro) -> 派生宏, 属性宏, 函数宏

// 声明宏 -> macro_rules! 宏名 { 规则 }
// #[macro_export]
macro_rules! vec_noah{
    // () 中的内容表示匹配模式
    // $x:expr 表示匹配的是一个表达式，匹配后的条目用 $x 代替
    // * 表示前面这个模式可以重复 0 次或者 1 次以上
    // 这个模式就是 $( $x:expr ),  注意后面还有一个逗号, 在匹配的时候是一个可选项，有就匹配，遇到最后一个匹配项的时候，就忽略它

    ($ ($x:expr), *) => {
        {
            let mut temp_vec = Vec::new();
            // $() 号和 => 前面那个 $() 的作用差不多，就是表明被包起来的这块代码是可重复的。
            // 紧跟的 * 表示这个代码块可以重复 0 次到多次
            // 具体次数等于 => 前面的 *号 所代表的次数
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    }
}
// 调用 vec_noah![1, 2, 3] 宏 展开后的结果
// {
//     let mut temp_vec = Vec::new();
//     temp_vec.push(1);
//     temp_vec.push(2);
//     temp_vec.push(3);
//     temp_vec
// }

// 常见的匹配方式
// expr: 匹配表达式
// ty: 匹配类型
// stmt: 匹配语句
// item: 匹配一个 item
// ident: 匹配一个标识符
// path: 匹配一个 path
// tt: 匹配一个 token tree
// 完整可以看: https://doc.rust-lang.org/stable/reference/macros-by-example.html

// 常见的重复符号
// *: 重复 0 次或者多次
// +: 重复 1 次或者多次
// ?: 重复 0 次或者 1 次

// 实现加法宏(自己实现)
// 可以使用 #[macro_export] 导出, 提供给其他模块使用
macro_rules! add {
    // 第一个分支, 匹配两个元素的加法
    ($a: expr, $b: expr) => {{
        $a + $b
    }};
    // 第二个分支: 当只有一个元素时, 也应该处理, 这是边界情况
    ($a:expr) => {{
        $a
    }};
}

// 拓展到多个数字的加法
macro_rules! add_multi {
    ( $($a:expr),* ) => {
       {
         // 开头要有个0，处理没有任何参数传入的情况
         0
         // 重复的块
         $( + $a )*
       }
    };
}

// 导出宏给其他模块使用
// 可以使用 #[macro_export] 导出, 提供给其他模块使用
// 在其他 crate 中可以使用 use crate_name::macro_name 导入
mod inner {
    super::m!();
    crate::m!();
}
mod toexport {
    #[macro_export] // 请注意这一句，把m!()导出到当前crate root下了
    macro_rules! m {
        () => {};
    }
}
fn _foo() {
    self::m!(); // main函数在当前crate根下，可这样调用m!()
    m!(); // 直接调用也是可以的
}

// macro_use (Rust 早期遗留写法, 更加推荐用到哪个导入哪个)
// 一次导入 一个 crate 中所有 导出的宏
// #[macro_use] extern crate rocket;

// 过程宏之 派生宏
use std::io;
use thiserror::Error;

#[derive(Error, Debug)] // 派生宏, Debug 宏由 std 提供，Error 由 thiserror crate 提供
pub enum DataStoreError {
    #[error("data store disconnected")] // 属性宏
    Disconnect(#[from] io::Error), // 属性宏
    #[error("the data for key `{0}` is not available")] // 属性宏
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")] // 属性宏
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")] // 属性宏
    Unknown,
}

// 过程宏之 属性宏
// Rust 编译器提供了一些属性（Attributes），属性是施加在模块、crate 或 item 上的元数据
// 代码条件编译；
// 设置 crate 名字，版本号和类型（是二进制程序还是库）；
// 禁止代码提示；
// 开启编译器特性（宏、全局导入等）；
// 链接到外部库；
// 标记函数为单元测试；
// 标记函数为性能评测的一部分；
// 属性宏；

// 要把属性施加到整个 crate，语法是在 crate 入口，比如在 lib.rs 或 main.rs 的第一行写上 #![crate_attribute]
// 如果只想把属性施加到某个模块或者 item 上，就把 ! 去掉

// 属性上还可以携带参数
// #[attribute = "value"]
// #[attribute(key = "value")]
// #[attribute(value)]

// 相关资料: https://doc.rust-lang.org/reference/attributes.html#attributes

// 具体例子

// 声明这个 crate 为 lib，是全局性的属性
// #![crate_type = "lib"]

// 声明下面这个函数为单元测试函数，这个属性只作用在test_foo()函数上
#[test]
fn test_foo() {
    /* ... */
}

// 条件编译属性，这块深入下去细节非常多
#[cfg(target_os = "linux")]
mod bar {
    /* ... */
}

// 正常来说，Rust中的类型名需要是Int8T这种写法，下面这个示例让编译器不要发警告
#[allow(non_camel_case_types)]
#[allow(unused)]
type int8_t = i8;

// 作用在整个函数内部，对未使用的变量不要报警
#[allow(unused)]
fn some_unused_variables() {
    #![allow(unused_variables)]

    let x = ();
    let y = ();
    let z = ();
}

// 属性宏 和 函数宏用到的时候再研究(高级)


// 宏的作用
// 1. 减少重复代码
// 2. 为类型添加额外能力 (派生宏 和 属性宏)
// 3. 创建 DSL(特定领域的新的语言)
// 4. 变换代码实现任意自定义目标

// 使用宏不能过度，宏的缺点是比较难调试，IDE 对它的支持可能也不完美。滥用宏会导致代码难以理解
fn main() {
    // 在 rust 中, 使用 [], (), {} 都可以
    // []: 通常用来表示 Vec 这种列表
    // (): 类函数式表用
    // {}: 构建结构体之类的宏 或者 存在大段代码输入的情况
    let _v = vec_noah![1, 2, 3];
    let _sum = add!(1, 2);
    let _sum = add!(1);
    let _sum = add_multi!();
    let _sum = add_multi!(1, 2, 3, 4, 5);
}
