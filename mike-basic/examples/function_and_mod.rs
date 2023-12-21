// 函数定义
fn print_a_b(a: i32, b: char) {
    println!("The value of a b is: {a}{b}");
}

fn main() {
    // 函数调用
    println!("***** 函数调用 *****");
    print_a_b(1, '2');
    // 标准的函数定义
    // fn  add_one_v1   (x: u32) -> u32 { x + 1 }

    // 闭包的定义，请注意形式对比
    // let add_one_v2 = |x: u32| -> u32 { x + 1 };

    // 闭包的定义2，省略了类型标注
    // let add_one_v3 = |x|             { x + 1 };

    // 闭包的定义3，花括号也省略了
    // let add_one_v4 = |x|              x + 1  ;

    // 闭包
    println!("\n***** 闭包 *****");
    let add_one = |x| x + 1;
    let a_vec: Vec<u32> = vec![1, 2, 3, 4, 5];
    let vec2: Vec<_> = a_vec.iter().map(add_one).collect();
    println!("{:?}", vec2);

    let a = 10u32; // 局部变量

    // fn  add_v1   (x: u32) -> u32 { x + a }  // 报错!  // 定义一个内部函数
    let add_v2 = |x: u32| -> u32 { x + a }; // 定义一个闭包

    // let result1 = add_v1(20); // 调用函数
    let result2 = add_v2(20); // 调用闭包
    println!("{}", result2);
}
