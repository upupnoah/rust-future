fn main() {
    // if else 语句
    println!("***** if else *****");

    let number = 6;
    // 判断数字number能被4，3，2中的哪一个数字整除
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // 类似于三元运算符
    let x = 1;
    // 在这里，if else 返回了值
    let y = if x == 0 {
        // 代码块结尾最后一句不加分号，表示把值返回回去
        100
    } else {
        // 代码块结尾最后一句不加分号，表示把值返回回去
        101
    };
    println!("y is {}", y);

    // 循环 loop
    println!("\n***** loop *****");
    let mut counter = 0;

    // 这里，接收从循环体中返回的值，对result进行初始化
    let result = loop {
        counter += 1;
        if counter == 10 {
            // 使用break跳出循环，同时带一个返回值回去
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // while 循环
    println!("\n***** while *****");
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    // for 循环
    println!("\n***** for *****");
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // for 循环，使用 in 关键字
    println!("\n***** for in *****");
    // 左闭右开区间
    for number in 1..4 {
        println!("{number}");
    }
    println!("--");
    // 左闭右闭区间
    for number in 1..=4 {
        println!("{number}");
    }
    println!("--");
    // 反向
    for number in (1..4).rev() {
        println!("{number}");
    }
}
