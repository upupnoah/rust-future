// 借用与引用

// Note (by noah)
// 引用语法 & 与 &mut ,解引用 *

// 所有权型变量, 引用型变量

// 引用的作用域-> 从声明开始到最后一次使用的地方
// 引用的限制
//   1. 对同一个 所有权型变量 的 可变引用 的作用域不能交叠(不能同时存在两个可变引用)
//   2. 对同一个 所有权型变量 的 可变引用 与 不可变引用 的作用域不能交叠(不能同时存在一个可变引用和一个不可变引用)

// 在一个所有权型变量有借用(可变/不可变)的情况下, 不能直接修改这个所有权型变量的值(可间接-> 可变引用 解引用)

// 将可变引用赋值给另一个变量, 会导致原来的可变引用失效(可以看成与所有权型变量一样,会发生所有权转移)
//     move

// 多级引用
//  1. 解引用修改值(注意层数)
//  2. println! 会自动解引用找到最终的值
//  3. 如果要通过多级引用修改值,需要保证中间的引用都是可变引用
fn main() {
    // 引用
    println!("***** 引用 *****");
    let a = 10u32;
    let b = &a; // b是变量a的一级引用
    let c = &&&&&a; // c是变量a的多级引用
    let d = &b; // d是变量a的间接引用
    let e = b; // 引用b再赋值给e

    println!("{a}");
    println!("{b}");
    println!("{c}");
    println!("{d}");
    println!("{e}");

    // 可变引用(reference)
    println!("\n***** 可变引用 *****");
    // let a = 10u32;
    let mut a = 10u32;
    let b = &mut a;
    *b = 20;
    // println!("{a}"); // 放前面会报错 println! 会对所有权变量做不可变借用
    println!("{b}");

    let mut a = 10u32;
    let b = &mut a;
    *b = 20;
    let c = &a;
    // println!("{b}"); // 打印 b 会报错
    println!("{c}"); // 加了一句打印语句

    // 可变引用与不可变引用同时存在
    println!("\n***** 可变引用与不可变引用同时存在 *****");
    let mut a1 = 10u32;
    let mut a2 = 15u32;
    let mut _b = &mut a1;
    _b = &mut a2;
    let mut _c = &a1;
    _c = &a2;

    // 多级引用
    println!("\n***** 多级引用1 *****");
    let mut a1 = 10u32;
    let mut b = &mut a1;
    *b = 20;
    let c = &mut b;
    **c = 30; // 两个*号表示解引用两次 (第一次解出来是&mut, 第二次解是 a1)
    println!("{c}");

    println!("\n***** 多级引用2 错误例子 *****");
    let mut a1 = 10u32;
    let b = &mut a1;
    let mut c = &b;
    let d = &mut c;
    // ***d = 30; // 错误,因为路径中间有不可变引用
    println!("{d}"); // println可以自动解引用

    // 不需要将s的所有权转移出来了
    println!("\n***** 使用&mut传入函数, 不需要将s的所有权转移出来啦! *****");
    fn foo(s: &mut String) {
        s.push_str(" You are batman.");
    }

    let mut s1 = String::from("I am a superman.");
    println!("{s1}");
    foo(&mut s1); // 注意这里传的是字符串的可变引用 &mut s1
    println!("{s1}");

    // 思考题1
    // 问题: 为啥在不可变引用存在的情况下, 原变量不能被修改呢?
    // 我的解答: 因为如果可以修改原变量, 那么对于不可变引用来说相当于是可变了, 失去了不可变引用的意义
    // let mut a: u32 = 10;
    // let b = &a;
    // a = 20;
    // println!("{}", b);

    // 思考题2
    // 问题: 可变引用复制的时候，为什么不允许 copy，而是 move?
    // 我的解答:
    //        如果是copy, 那么会发生两个可变引用同时存在的情况, 违反了可变引用的规则
}