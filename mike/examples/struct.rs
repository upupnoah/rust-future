fn main() {
    // 结构体
    struct User {
        _active: bool,
        _username: String,
        _email: String,
        _sign_in_count: u64,
    }
    let _user1 = User {
        _active: true,
        _username: String::from("someusername123"),
        _email: String::from("someone@exmaple.com"),
        _sign_in_count: 1,
    };
    struct _Class {
        serial_number: u32,
        grade_number: u32,
        entry_year: String,
        members: Vec<User>, // 复杂结构体
    }

    // 三种结构体: 命名结构体, 元组结构体 和 单元结构体
    println!("\n***** 命名结构体 *****");
    let _active = true;
    let _username = String::from("someusername123");
    let _email = String::from("someone@example.com");
    let mut _user1 = User {
        _active, // 相当于 active: active
        _username,
        _email,
        _sign_in_count: 1,
    };
    // println!("{}", username); // 因为username是一个String, 上面复制给结构体的时候发生了所有权转移
    _user1._email = String::from("anotheremail@example.com"); // 只有mut修饰的结构体变量才能修改
    let _user2 = User {
        _email: String::from("newemail"),
        // 当结构体比较大的时候, 这种方法可以保持代码干净清爽
        .._user1 // 使用..语法从其他实例中获取剩余的值
    };

    // 用户的信息存在数据库里，当我们要更新一个用户的一个字段的信息时，
    // 首先需要从数据库里把这个用户的信息取出来，做一些基本的校验，然后把要更新的字段替换成新的内容
    // 再把这个新的用户实例存回数据库
    // 伪代码
    // let user_id = get_id_from_request;
    // let new_user_name = get_name_from_request();
    // let old_user: User = get_from_db(user_id);
    // let new_user: User = User {
    //     username: new_user_name,
    //     ..old_user // 注意这里的写法
    // };
    // new_user.save()

    // 元组结构体
    // 方便表示不同的示例: Color, Point
    println!("\n***** 元组结构体 *****");
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // 单元结构体
    // 只有类型名字, 没有任何字段
    // 其实它就相当于定义了一种类型，它的名字就是一种信息，有类型名就可以进行实例化，承载很多东西
    struct ArticleModule;
    let _article_module = ArticleModule; // 创建实例

    // 结构体中的所有权问题
    #[derive(Debug)]
    struct TestUser {
        _active: bool,
        _username: String,
        email: String,
        _sign_in_count: u32,
    }
    let _active = true;
    let _testuser = TestUser {
        _active,
        _username: String::from("someusername123"),
        email: String::from("example@xx.com"),
        _sign_in_count: 1,
    };
    let _email_clone = _testuser.email.clone();
    let _email_ref = &_testuser.email;
    let _email = _testuser.email; // 这里会发生所有权转移 (结构体中的email)

    // println!("{:?}", _testuser); // 这里会报错, 因为部分所有权已经转移, 分别打印另外3个是ok的

    // 引用类型的结构体字段
    // struct User { // 暂时无法通过编译
    //     active: &bool,       // 这里换成了 &bool
    //     username: &str,      // 这里换成了 &str
    //     email: &str,         // 这里换成了 &str
    //     sign_in_count: &u32, // 这里换成了 &u32
    // }

    // 结构体加标注
    // #[derive(Debug)] // 这个属性标注可以让我们使用 {:?} 打印结构体

    // 方法(实例方法)
    println!("\n***** 方法(实例方法) *****");
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        // 就像这样去实现
        fn area(self) -> u32 {
            // 这里的self是语法糖, 作用与 this 类似
            // area就是方法，被放在impl实现体中
            self.width * self.height
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle(矩形) is {} square pixels.",
        rect1.area() // 使用点号操作符调用area方法
    );

    impl Rectangle {
        fn _area1(self: Self) -> u32 {
            // 等价于 fn area1(self)
            self.width * self.height
        }
        fn _area2(self: &Self) -> u32 {
            // 等价于 fn area2(&self)
            self.width * self.height
        }
        fn _area3(self: &mut Self) -> u32 {
            // 等价于 fn area3(&mut self)
            self.width * self.height
        }
    }
    // rect1.area1(); // 传入rect1
    // rect1.area2(); // 传入&rect1
    // rect1.area3(); // 传入&mut rect1

    // 实例的引用也是可以直接调用方法的
    // 对同一个类型，impl 可以分开写多次

    // 关联函数 (静态方法)
    // 通过 :: 调用
    // impl 内的函数, 通过是否有self来区分是不是关联函数
    println!("\n***** 关联函数 (静态方法) *****");
    impl Rectangle {
        fn _numbers(rows: u32, cols: u32) -> u32 {
            rows * cols
        }
    }
    // 结构体名::关联函数名
    Rectangle::_numbers(1, 2); // 调用关联函数

    // 构造函数
    // Rust社区一般约定使用 new() 这个名字的关联函数来作为构造函数
    // 也有 from(), from_xxx() 等构造函数
    println!("\n***** 构造函数 *****");
    impl Rectangle {
        pub fn new(width: u32, height: u32) -> Self {
            Rectangle { width, height }
        }
    }
    let _rect1 = Rectangle::new(30, 50); // 调用构造函数

    // Default 派生宏
    println!("\n***** Default 派生宏 *****");
    #[derive(Debug, Default)] // 这里加了一个Default派生宏
    struct Rectangle1 {
        _width: u32,
        _height: u32,
    }
    let rect1: Rectangle1 = Default::default(); // 使用方式1
    let rect2 = Rectangle1::default(); // 使用方式2 println!("{:?}", rect1); println!("{:?}", rect2);
    println!("{:?}", rect1);
    println!("{:?}", rect2);

    // 但是，对于我们特定场景的 Rectangle 这种，我们可能希望给它赋一个初始的非 0 值
    // 在 Rust 中，这可以做到，但是需要用到后面的知识。目前我们就可以先用约定的 new 关联函数 + 参数来达到我们的目的
    println!("\n***** Default 非0值, 暂时通过 new *****");
    #[derive(Debug)]
    struct RectangleNew {
        _width: u32,
        _height: u32,
    }
    impl RectangleNew {
        pub fn new(_width: u32, _height: u32) -> Self {
            RectangleNew { _width, _height }
        }
    }
    const INITWIDTH: u32 = 50;
    const INITHEIGHT: u32 = 30;

    // 创建默认初始化值的Rectangle实例
    let rect1 = RectangleNew::new(INITWIDTH, INITHEIGHT);
    println!("rect1 is {:?}", rect1);

    // 思考题: 可以对 i8 类型做 impl 吗?
    // impl i8 {
    //     fn _add(self, other: i8) -> i8 {
    //         self + other
    //     }
    // }
    // 目前为止还不行
    // 后面可以使用trait 或者 newtype pattern
}