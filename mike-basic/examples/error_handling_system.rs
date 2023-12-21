use std::io::{self, Error, ErrorKind};

// std 中的错误类型
// 1. std::io::Error -> 负责表示标准库里 I/O 相关场景的错误类型
//     常见的 I/O 操作指的是标准输入输出、网络读写、文件读写等。
//     在 std::io 模块中，对这种 I/O 上的读写做了统一的抽象，而类型 io::Error 也是这个抽象里的一部分
// 2. parseError (和 FromStr trait 相关的错误类型)
//      2.1 std::num::ParseIntError
//      2.2 std::num::ParseFloatError
//      2.3 std::char::ParseCharError
//      2.4 std::str::ParseBoolError
//      2.5 std::net::AddrParseError

fn create_io_error() {
    println!("****** create_io_error ******\n");
    let _custom_error = Error::new(ErrorKind::Other, "oh no!");
}

// 和 FromStr trait 相关的错误类型
fn parse_error() {
    println!("\n****** parse_error ******\n");
    use std::net::IpAddr;
    let s = "1000eee";
    if let Err(e) = s.parse::<i32>() {
        // e 这里是 ParseIntError 类型
        println!("Failed conversion to i32: {e}.");
    }
    let addr = "127.0.0.1:8080".parse::<IpAddr>();
    if let Err(e) = addr {
        // e 这里是 AddrParseError 类型
        println!("Failed conversion to IpAddr: {e}.");
    }
}

// 用枚举定义错误
fn use_enum_to_define_error() {
    println!("\n****** use_enum_to_define_error ******\n");
    enum HereError {
        _Error1,
        _Error2,
        Error3,
    }
    // 一个函数返回 Err
    fn bar() -> Result<String, HereError> {
        Err(HereError::Error3)
    }
    fn foo() {
        match bar() {
            Ok(_) => {}
            Err(err) => match err {
                // 在上层中通过 match 进行处理
                HereError::_Error1 => {}
                HereError::_Error2 => {}
                HereError::Error3 => {
                    // do something
                }
            },
        }
    }
    foo()
}

// 函数返回 Result<T, E>
fn result_t_e() {
    println!("\n****** result_t_e ******\n");
    fn _foo1(num: u32) -> Result<String, String> {
        if num == 10 {
            Ok("Hello world!".to_string())
        } else {
            Err("I'm wrong!".to_string())
        }
    }
    fn _foo2(num: u32) -> Result<String, u32> {
        if num == 10 {
            Ok("Hello world!".to_string())
        } else {
            Err(100)
        }
    }

    // 有时一个函数中的错误情况可能不止一种，这时候该怎样定义返回类型呢？惯用办法就是使用 enum
    enum _MyError {
        Error1,
        Error2,
        Error3,
    }

    fn _foo3(num: u32) -> Result<String, _MyError> {
        match num {
            10 => Ok("Hello world!".to_string()),
            20 => Err(_MyError::Error1),
            30 => Err(_MyError::Error2),
            _ => Err(_MyError::Error3),
        }
    }
}

// Result<_, Box<dyn Error>
fn result_box_dyn_error() {
    println!("\n****** result_box_dyn_error ******\n");
    use std::error::Error;
    use std::fmt;

    #[derive(Debug)]
    struct MyError;

    impl fmt::Display for MyError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "MyError")
        }
    }

    impl Error for MyError {}

    fn foo(num: u32) -> Result<String, Box<dyn Error>> {
        match num {
            10 => Ok("Hello world!".to_string()),
            _ => {
                let my_error = MyError;
                // 因为 MyError 实现了 Error trait, 因此可以作为 dyn Error 的实现
                Err(Box::new(my_error))
            }
        }
    }
    println!("{}", foo(10).unwrap());
    println!("{}", foo(20).unwrap_err());
}

// map_err 转换错误类型
// 很多时候同一个函数中会产生不同的错误类型，这时仍然可以使用 map_err 显式地把不同的错误类型转换成我们需要的同一种错误类型
fn map_err() {
    println!("\n****** map_err ******\n");

    use std::fs::File;
    use std::io::Read;

    fn read_file() -> Result<String, String> {
        // 使用 map_err 将这两个 I/O 错误的类型都转换成了 String 类型
        match File::open("./examples/foo.txt").map_err(|err| format!("Error opening file: {}", err))
        {
            Ok(mut file) => {
                let mut contents = String::new();
                match file
                    .read_to_string(&mut contents)
                    .map_err(|err| format!("Error reading file: {}", err))
                {
                    Ok(_) => Ok(contents),
                    Err(e) => return Err(e),
                }
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    read_file().unwrap();

    // 使用 ?
    fn _read_file() -> Result<String, String> {
        let mut file = File::open("./examples/foo.txt")
            .map_err(|err| format!("Error opening file: {}", err))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|err| format!("Error reading file: {}", err))?;
        Ok(contents)
    }
}

// Result 链式处理

// map_err()：当 Result 是 Ok 时，传递原样返回。当 Result 是 Err 时，对 Err 携带的内容使用这个方法提供的函数或闭包进行运算及类型转换。
// 这个方法常常用于转换 Result 的 Err 的负载的类型，在错误处理流程中大量使用

// and_then()：如果 Option 是 None，返回 None；如果 Option 是 Some，
// 就把参数里面提供的函数或闭包应用到被包裹的内容上，并返回运算后的结果

// map()：在 Option 是 Some 的情况下，通过 map 中提供的函数或闭包把 Option 里的类型转换成另一种类型。
// 在 Option 是 None 的情况下，保持 None 不变。map() 会消耗原类型，也就是获取所有权
fn result_chain_processing() {
    println!("\n****** result_chain_processing ******\n");
    use std::fs::File;
    use std::io::Read;

    // 这里用到了 map_err、and_then、map 三种链式操作，它们可以在不解开 Result 包的情况下直接对里面的内容进行处理
    fn read_file() -> Result<String, String> {
        File::open("./examples/foo.txt")
            // 如果File::open() 是 Ok, 则跳过下面这行代码, 直接执行 and_then() 中的代码
            .map_err(|err| format!("Error opening file:{}", err))
            // 会消解前面产生的 Result，把 file 对象传进来使用
            .and_then(|mut file| {
                let mut contents: String = String::new();
                file.read_to_string(&mut contents)
                    .map_err(|err| format!("Error reading file: {}", err))
                    // 将 contents 字符串 move 进来作为内层闭包的返回值，并进一步以 Ok(contents) 的形式作为 read_file() 函数的返回值返回
                    .map(|_| contents)
            })
    }
    match read_file() {
        Ok(contents) => println!("{}", contents),
        Err(e) => println!("{}", e),
    }
}

// ? 问号操作符
// let ret = a_result?;
// 等价于:
// let ret = match a_result {
//     Ok(ret) => ret,
//     Err(e) => return Err(e), // 注意这里有一个return语句。
// };
// 这实际是一种防御式编程，遇到了错误，就提前返回
fn question_mark_operator() {
    println!("\n****** question_mark_operator ******\n");
    use std::fs::File;
    use std::io::Read;

    fn read_file() -> Result<String, String> {
        let mut file = File::open("./examples/foo.txt")
            .map_err(|err| format!("Error opening file: {}", err))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|err| format!("Error reading file: {}", err))?;
        Ok(contents)
    }
    println!("{}", read_file().unwrap());
    // match read_file() {
    //     Ok(contents) => println!("{}", contents),
    //     Err(e) => println!("{}", e),
    // }

    // ------------------------------
    // 改成不使用 map_err() 的写法

    // 由于 trait 的孤儿规则,  Rust 要求在一个模块中，如果要对一个类型实现某个 trait，这个类型和这个 trait 其中必须有一个是在当前模块中定义的
    // 解决方法: 用 newtype 方法定义了一个新的错误类型 (包一下), 实际上就是 元组结构体
    struct MyError(String);
    impl From<std::io::Error> for MyError {
        fn from(err: std::io::Error) -> Self {
            MyError(format!("Error: {}", err))
        }
    }
    // 将 read_file() 函数的返回值类型从 Result<String, String> 改成 Result<String, MyError>
    fn _read_file() -> Result<String, MyError> {
        // 由于为 MyError 实现了 From<std::io::Error> trait，因此这里的 ? 会自动将 std::io::Error 转换成 MyError
        let mut file = File::open("./examples/foo.txt")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
}

// 错误的表示 最佳实践: thiserror 库

// 错误的传递 最佳实践:  使用?, 遇到不一样的:
//                  1. map_err, 手动实现 From<T> trait
//                  2. 或者利用 thiserror 里提供的 #[from] 属性宏标注

// 错误处理 最佳实践: anyhow crate

fn use_thiserror() {
    println!("\n****** use_thiserror ******\n");

    use thiserror::Error; // 引入宏

    #[derive(Error, Debug)] // derive Error micro
    pub enum DataStoreError {
        // error 是 attribute
        #[error("data store disconnected")] // 属性标注
        // #[from] 的作用是将源错误类型转换成目标错误类型
        // 将 io::Error 转换成 DataStoreError
        Disconnect(#[from] io::Error), // 属性标注

        #[error("the data for key `{0}` is not available")]
        _Redaction(String),
        #[error("invalid header (expected {expected:?}, found {found:?})")]
        _InvalidHeader { expected: String, found: String },
        #[error("unknown data store error")]
        _Unknown,
    }
}
// 可以统一使用 Result<T, anyhow::Error> 或 等价的 anyhow::Result<T> 来表示函数的返回值类型
// 使用 anyhow::Result 作函数返回值，你在函数中可以使用 ？操作符来把错误向上传递，
// 只要这个错误类型实现了 std::error::Error 这个 trait 就行了

// std::error:Error 这个 trait 是std 标准库中的一个标准类型, 想让让自己错误融入社区, 需要实现这个 trait
fn use_anyhow() {
    println!("\n****** use_anyhow ******\n");
    use anyhow::Result;
    use std::fs::File;
    use std::io::Read;
    fn read_file() -> Result<String> {
        let mut file = File::open("./examples/foo.txt")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents + " anyhow!!")
    }
    // println!("{}", read_file().unwrap());
    match read_file() {
        Ok(contents) => println!("{}", contents),
        Err(e) => {
            // 获取错误根源
            let root_cause = e.root_cause();
            match root_cause.downcast_ref::<io::Error>() {
                // io::Error 类型的错误
                Some(io_error) => {
                    println!("IO error: {}", io_error);
                }
                // 非 io::Error 类型的错误
                None => {
                    eprintln!("An error occurred: {:?}", e);
                }
            }
        }
    }
}
fn main() {
    create_io_error();

    parse_error();

    use_enum_to_define_error();

    result_t_e();

    result_box_dyn_error();

    map_err();

    result_chain_processing();

    question_mark_operator();

    use_thiserror();

    use_anyhow();
}
