fn main() {
    // 元组 tuple
    println!("***** 元组 tuple *****");
    let x: (i32, f64, u8) = (500, 6.4, 1);

    // 元组使用.运算符访问其元素，下标从0开始，注意语法
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!(
        "five_hundred = {}, six_point_four = {}, one = {}",
        five_hundred, six_point_four, one
    );

    // 结构体 struct
    println!("\n***** 结构体 struct *****");
    struct User {
        // 定义结构体
        active: bool,
        username: String,
        email: String,
        age: u64,
    }
    let user1 = User {
        // 创建结构体实例
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        age: 1,
    };
    println!(
        "user1.active = {}\n user1.username = {}\n user1.email = {}\n user1.age = {}\n",
        user1.active, user1.username, user1.email, user1.age
    );

    // 枚举 enum
    println!("\n***** 枚举 enum *****");

    #[derive(Debug)] // Rust 的派生功能自动为 IpAddrKind 实现 Debug trait
    enum IpAddrKind {
        // 定义枚举
        V4,
        V6,
    }
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("four = {:?}, six = {:?}", four, six);
}
