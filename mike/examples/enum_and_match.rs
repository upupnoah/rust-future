// 枚举 enum
fn enum_example() {
    println!("enum");
    enum _Shape {
        Rectangle,
        Triangle,
        Circle,
    }

    enum _Shape1 {
        Rectangle { width: u32, height: u32 },
        Triangle((u32, u32), (u32, u32), (u32, u32)),
        Circle { origin: (u32, u32), radius: u32 },
    }

    // 将结构体挂载到枚举类型上
    struct _Rectangle {
        width: u32,
        height: u32,
    }

    enum _Shape2 {
        Rectangle(_Rectangle),
        // ...
    }
}

// enum的 实例化
fn enum_instance() {
    println!("\n***** enum的 实例化 *****");

    #[derive(Debug)]
    enum WebEvent {
        PageLoad,
        PageUnload,
        KeyPress(char),
        Paste(String),
        _Click { _x: i64, _y: i64 },
    }
    let _a = WebEvent::PageLoad;
    let _b = WebEvent::PageUnload;
    let _c = WebEvent::KeyPress('c');
    let _d = WebEvent::Paste(String::from("batman"));
    let _e = WebEvent::_Click { _x: 320, _y: 240 };
}
// 类C枚举
fn c_like_enumlation() {
    println!("\n***** 类C枚举 *****");
    // 给枚举变体一个起始数字值
    enum Number {
        Zero = 0,
        One,
        _Two,
    }

    // 给枚举每个变体赋予不同的值
    enum Color {
        Red = 0xff0000,
        _Green = 0x00ff00,
        Blue = 0x0000ff,
    }

    // 使用 as 进行类型的转化
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}

// 空枚举
fn empty_enum() {
    println!("\n***** 空枚举 *****");
    enum _Foo {} // 这种形式目前没啥用, 只需要了解可以这样子定义即可
                 // let a = Foo {}; // error
}

// impl 枚举
fn impl_enum() {
    println!("\n***** impl 枚举 *****");
    enum MyEnum {
        Add,
        _Subtract,
    }
    impl MyEnum {
        fn run(&self, x: i32, y: i32) -> i32 {
            match self {
                Self::Add => x + y,
                Self::_Subtract => x - y,
            }
        }
    }
    let add = MyEnum::Add;
    println!("add.run(100, 200) = {}", add.run(100, 200));

    // 不能直接对枚举的变体 impl
    // enum Foo {
    //     AAA,
    // }
    // impl Foo:AAA {}; // error
}

// match
fn match_example() {
    println!("\n***** match *****");
    #[derive(Debug)]
    enum Shape {
        Rectangle,
        _Triangle,
        _Circle,
    }

    let shape_a = Shape::Rectangle; // 创建实例
    match shape_a {
        // 匹配实例
        Shape::Rectangle => {
            println!("{:?}", Shape::Rectangle); // 进了这个分支
        }
        Shape::_Triangle => {
            println!("{:?}", Shape::_Triangle);
        }
        Shape::_Circle => {
            println!("{:?}", Shape::_Circle);
        }
    }
}

// match 可返回值
fn match_return_value() {
    println!("\n***** match 可返回值 *****");
    #[derive(Debug)]
    enum Shape {
        Rectangle,
        _Triangle,
        _Circle,
    }

    let shape_a = Shape::Rectangle; // 创建实例
    let ret = match shape_a {
        // 匹配实例，并返回结果给ret
        Shape::Rectangle => 1,
        Shape::_Triangle => 2,
        Shape::_Circle => 3,
    };
    println!("{}", ret);
}

// match 处理分支
// rust match 必须处理所有分支, 如果带有返回值, 返回值类型必须一致
fn match_branch() {
    #[derive(Debug)]
    enum Shape {
        _Rectangle,
        _Triangle,
        _Circle,
    }

    let shape_a = Shape::_Rectangle;
    let ret = match shape_a {
        Shape::_Rectangle => 1,
        _ => 10, // 使用 _ 处理剩余分支 (占位符)
    };
    println!("{}", ret);
}

// 更加广泛的分支
fn extensive_branch() {
    let number = 13;
    // 你可以试着修改上面的数字值，看看下面走哪个分支

    println!("Tell me about {}", number);
    match number {
        // 匹配单个数字
        1 => println!("One!"),
        // 匹配几个数字
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // 匹配一个范围，左闭右闭区间
        13..=19 => println!("A teen"),
        // 处理剩下的情况
        _ => println!("Ain't special"),
    }
}

// 模式匹配
fn pattern_match() {
    println!("\n***** 模式匹配 if let | while let | let *****");
    #[derive(Debug)]
    enum Shape {
        Rectangle,
        _Triangle,
        Circle,
    }
    let shape_a = Shape::Rectangle;
    // match shape_a {
    //     Shape::Rectangle => {
    //         println!("1");
    //     }
    //     _ => {
    //         println!("10");
    //     }
    // };

    // if let, 上面注释代码的改写
    if let Shape::Rectangle = shape_a {
        println!("1");
    } else {
        println!("10");
    }

    // while let
    let mut shape_a = Shape::Rectangle;
    let mut i = 0;
    while let Shape::Rectangle = shape_a {
        // 注意这一句
        if i > 9 {
            println!("Greater than 9, quit!");
            shape_a = Shape::Circle;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            i += 1;
        }
    }

    // let 本身就支持模式匹配
    #[derive(Debug)]
    enum Shape1 {
        Rectangle { width: u32, height: u32 },
        _Triangle,
        _Circle,
    }
    // 创建实例
    let shape_a = Shape1::Rectangle {
        width: 10,
        height: 20,
    };
    // 模式匹配出负载内容,  匹配结构体负载，获取字段值
    // 同时定义了 width 和 height 两个局部变量，并初始化为枚举变体的实例负载的值。这两个局部变量在后续的代码块中可以使用
    let Shape1::Rectangle { width, height } = shape_a else {
        panic!("Can't extract rectangle.");
    };
    println!("width: {}, height: {}", width, height);

    // 匹配元组 (元组析构): 常用来从函数的多个返回值中取出数据
    fn foo() -> (u32, u32, char) {
        (1, 2, 'a')
    }

    let (b, c, d) = foo();
    println!("b: {}, c: {}, d: {}", b, c, d);
}

// 匹配枚举
fn match_enum() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    enum Shape {
        _Rectangle(Rectangle),
        _Triangle((u32, u32), (u32, u32), (u32, u32)),
        Circle { origin: (u32, u32), radius: u32 },
    }

    let _a_rec = Rectangle {
        width: 10,
        height: 20,
    };

    // 请打开下面这一行进行实验
    // let shape_a = Shape::_Rectangle(_a_rec);
    // 请打开下面这一行进行实验
    let _shape_a = Shape::_Triangle((0, 1), (3, 4), (3, 0));

    let shape_a = Shape::Circle {
        origin: (0, 0),
        radius: 5,
    };

    // 这里演示了在模式匹配中将枚举的负载解出来的各种形式
    match shape_a {
        Shape::_Rectangle(a_rec) => {
            // 解出一个结构体
            println!("Rectangle {}, {}", a_rec.width, a_rec.height);
        }
        Shape::_Triangle(x, y, z) => {
            // 解出一个元组
            println!("Triangle {:?}, {:?}, {:?}", x, y, z);
        }
        Shape::Circle { origin, radius } => {
            // 解出一个结构体的字段
            println!("Circle {:?}, {:?}", origin, radius);
        }
    }
}

// 匹配 结构体
fn match_struct() {
    println!("\n***** 匹配 结构体 *****");
    #[derive(Debug)]
    struct User {
        name: String,
        age: u32,
        student: bool,
    }

    let a = User {
        name: String::from("mike"),
        age: 20,
        student: false,
    };

    // Rust 中的模式匹配是一种释放原对象的所有权的方式
    // name 是 String, 会被移动
    let User { name, age, student } = a; // moved here

    println!("{}", name);
    println!("{}", age);
    println!("{}", student);
    // println!("{:?}", a); // error, partial move
}

// ref 关键字
fn ref_keyword() {
    println!("\n***** ref 关键字 *****");
    #[derive(Debug)]
    struct User {
        name: String,
        age: u32,
        student: bool,
    }

    let a = User {
        name: String::from("noah"),
        age: 20,
        student: false,
    };
    let User {
        ref name, // 这里加了一个ref, 使得name不会被移动
        age,
        student,
    } = a;

    // 可以正常打印
    println!("{}", name);
    println!("{}", age);
    println!("{}", student);
    println!("{:?}", a);
}

// match in function
fn match_in_function() {
    println!("\n***** match in function *****");
    // 例子1
    fn foo((a, b, c): (u32, u32, char)) {
        // 通过传入参数的人, 没有解析出3个“变量”
        // 注意这里的定义
        println!("{}", a);
        println!("{}", b);
        println!("{}", c);
    }

    let a = (1, 2, 'a');
    foo(a);

    #[derive(Debug)]
    struct User {
        name: String,
        age: u32,
        student: bool,
    }

    // 例子2
    fn foo1(
        User {
            // 注意这里的定义
            name,
            age,
            student,
        }: User,
    ) {
        println!("{}", name);
        println!("{}", age);
        println!("{}", student);
    }

    let a = User {
        name: String::from("mike"),
        age: 20,
        student: false,
    };
    foo1(a);
}

fn main() {
    enum_example();
    enum_instance();
    c_like_enumlation();
    empty_enum();
    impl_enum();
    match_example();
    match_return_value();
    match_branch();
    extensive_branch();
    pattern_match();
    match_enum();
    match_struct();
    ref_keyword();
    match_in_function();
}
