// Note (by noah)
// str 不能直接使用,一般是通过 &str 或者 String 来使用
// String 可以看作是 str 的动态、可增长版本

// &str || &[T] 都是不可片引用 切片
// &'static str: 特殊的&str, 是一个静态数据区的引用(程序生命周期内都有效的字符串切片)

// String <-> &str || &[u8]
//      String -> &str: &s[..]
//      String -> &str: s.as_str()
//      String -> &[u8]: s.as_bytes()
//      &str   -> String: String::from(&str)
//      &str   -> String: &str.to_string()
//      &str   -> String: &str.to_owned()
//      &str   -> &[u8]: &str.as_bytes()
//      &[u8]  -> String: String::from_utf8(&[u8])
//      &[u8]  -> String: String::from_utf8_unchecked(&[u8])
//      &[u8]  -> &str: str::from_utf8(&[u8])
//      &[u8]  -> &str: str::from_utf8_unchecked(&[u8])

// Vec<T> <-> &[T]
//      Vec<T> -> &[T]: &vec[..]
//      Vec<T> -> &[T]: vec.as_slice()
//      &[T]   -> Vec<T>: Vec::from(&[T])
//      &[T]   -> Vec<T>: &[T].to_vec()
//      &[T]   -> Vec<T>: &[T].to_owned()

// Deref Trait (隐式引用类型转换)
//      &String -> &str
//      &Vec<T> -> &[T]

// 其他字符串类型: Path、PathBuf、OsStr、OsString、CStr、CString

// Parse: str -> T
// 只要T实现了FromStr这个Trait，就可以从字符串转换到任意Rust类型
//  let name = str.parse::<T>() -> Result<T, E>
//  let name: Result<T, E> = str.parse()
// 一般使用unwarp()来获取T类型的值

// 序列化和反序列化的方案 -> serde
use std::num::ParseIntError;

fn main() {
    // 不同类型的字符串
    println!("***** 不同类型的字符串 *****");
    let _s1: &'static str = "I am noah."; // 特殊的&str, 是一个静态数据区的引用(程序生命周期内都有效的字符串切片)
    let _s2 = _s1.to_string(); // String
    let _s3 = &_s2; // &String String的不可变引用
    let _s4 = &_s2[..]; // &str String的切片
    let _s5 = &_s2[..6];

    // &str -> String
    // s = String::from("I am a noah.");
    //      1. "I am a noah.".to_string()
    //      2.  &s[..].to_string()
    println!("\n***** &str -> String *****");
    let _str = "&str";
    let _string = String::from("String");

    let s = "I am a superman.".to_string();
    let a_slice: &str = &s[..];
    let _another_string: String = a_slice.to_string();
    println!("s is {}", a_slice.chars().nth(0).unwrap());

    // String 修改值(safe): 效率低
    println!("\n***** String 修改值 *****");
    let mut s = String::from("this is a String");
    println!("s is {s}");
    // let char_to_insert = 'x'; // char
    let mut chars = s.chars().collect::<Vec<char>>();
    chars[1] = 'x';
    s = chars.into_iter().collect();
    println!("s is {s}");

    // 切片
    println!("\n***** 切片 *****");
    let s = String::from("abcdefg");
    // 将s的第二个字符设置为x
    s.chars().nth(2).unwrap().to_string();
    let _slice1 = &s[..];
    let _slice2 = &s[0..4];
    let _slice3 = &s[2..5];
    let _str_slice = &"abcdefg"[..];
    // T = [1,2,3]
    // T = "abcdefg"
    // let _x_slice = &T[..];

    // 字符串型切片(&str) -> [所有权型]字符串(String)
    //      字符串切片&str -> 字符串String
    //      &str 可以通过: to_string(), to_owned() 转换为 String
    println!("\n***** 字符串型切片(&str) -> [所有权型]字符串(String) *****");
    let s: &str = "I am a superman.";
    let _s1: String = String::from(s); // 使用 String 的from构造器
    let _s2: String = s.to_string(); // 使用 to_string() 方法
    let _s3: String = s.to_owned(); // 使用 to_owned() 方法

    // [u8], &[u8], &[u8;N], Vec<u8>
    // [u8]: 是指向某个字节数组的不可变切片的引用，可以是任意长度
    // &[u8;N]: 是指向一个具有固定大小 N 的字节数组的引用
    //      基本类型都可以通过: to_vec(), to_owned() 转换为 Vec<T>
    // Vec<u8>: 是一个动态、可增长的字节数组容器，它拥有自己的数据
    println!("\n***** [u8], &[u8], &[u8;N], Vec<u8> *****");
    let a_vec: Vec<u8> = vec![1, 2, 3];
    let a_slice = &a_vec[..];
    let _another_vec = a_slice.to_vec();
    let _another_vec = a_slice.to_owned();

    let fixed_array: [u8; 5] = [1, 2, 3, 4, 5]; // 固定大小的数组
    let _fixed_array_slice = &fixed_array; // 不可变引用到固定大小的数组
    let _byte_slice = &fixed_array[..]; // 不可变引用到不定长的数组切片
                                        // 默认是i32, 这里自动推导为u8
    let mut dynamic_byte_vector = Vec::new(); // 动态数组，可以增长和缩小
    dynamic_byte_vector.push(1);
    dynamic_byte_vector.push(2);
    dynamic_byte_vector.extend_from_slice(&fixed_array);

    // as_str(), as_bytes()
    //      String -> &str, &[u8]
    // as_slice() (相当于String的as_str())
    //      Vec<T> -> &[T]
    println!("\n***** String「as_str(), as_bytes()」 Vec<T>「as_slice()」 *****");
    let s = String::from("I am noah.");
    let _s1 = s.as_str();
    let _s2 = s.as_bytes();
    let vec1 = vec![1u8, 2, 3];
    let _vec2 = vec1.as_slice();

    // 隐式引用类型转换
    // String 和 &str 在细节下, 使用起来麻烦, 怎么解决?
    // 同样适用于 Vec<T> 和 &[T] (可以把String看成 Vec<str>, &str 看成 &[str])
    println!("\n***** String 和 &str 隐式引用类型转换 *****");
    // fn foo(_s: &String) {}
    let s = String::from("I am noah.");
    let _s1 = &s[..];
    // foo(&s);
    // foo(_s1); error: expected struct `String`, found `&str`
    // 修改为
    fn foo_new(_s: &str) {}
    foo_new(&s); // 隐式转换 &String -> &str, 为什么可以? Deref Trait
    foo_new(_s1);

    // &str 可以转成 &u8
    println!("\n***** &str 可以转成 &u8 *****");
    let bytes = "bors".as_bytes();
    assert!(b"bors" == bytes);

    // new knowledges
    // &[T]: 不可变 and 不定长数组切片
    // &[T;N]: 不可变 and 固定大小数组切片

    // 字符串 转换成 字符串
    // from_utf8 系列函数，返回的是 Result
    // String::from_utf8()
    // String::from_utf8_unchecked()
    // str::from_utf8()
    // str::from_utf8_unchecked()

    // 字符串切割成字符数组
    // String -> chars() -> collect() -> Vec<char>
    // String -> chars() -> for ch in s.chars() {}
    println!("\n***** 字符串切割成字符数组 *****");
    let s = String::from("你好Noah");
    let char_vec = s.chars().collect::<Vec<char>>();
    println!("char_vec is {:?}", char_vec);
    for ch in s.chars() {
        println!("ch is {}", ch);
    }

    // Vec<T> <-> &[T]
    println!("\n***** Vec<T> <-> &[T] *****");
    let vec1 = vec![1, 2, 3];
    let slice1 = &vec1[..];
    let slice2 = vec1.as_slice();
    let v1 = slice1.to_vec();
    let v2 = slice1.to_owned();
    println!("slice1 is {:?}", slice1);
    println!("slice2 is {:?}", slice2);
    println!("v1 is {:?}", v1);
    println!("v2 is {:?}", v2);

    // str Parse 方法
    // 可以从字符串转换到任意 Rust 类型，只要这个类型实现了 FromStr 这个 Trait
    println!("\n***** str Parse 方法 *****");
    let _a = "10".parse::<u32>();
    let _b: Result<u32, ParseIntError> = "10".parse();
    let _aa: u32 = "10".parse().unwrap();
    println!("a is {:?}", _a);
    let a = "4.2".parse::<f32>();
    println!("{:?}", a);

    // 思考题: chars 函数是定义在 str 上的，为什么 String 类型能直接调用 str 上定义的方法？
    // 实际上 str 上的所有方法，String 都能调用，请问这是为什么呢？
    println!("\n***** 思考题: chars 函数是定义在 str 上的，为什么 String 类型能直接调用 str 上定义的方法？ *****");
    let s = String::from("你好Noah");
    let _char_vec = s.chars().collect::<Vec<char>>();
    // s.chars() 可以看成 (&s).chars() -> (&String).chars()
    // 因为 String 实现了 Deref<Target=str> Trait, 所以 String 可以调用 str 上的方法
    // 具体操作: 如果String自身没有方法, rust会通过Deref<Target=str> Trait, 在str上查找方法
    // 自己的理解: &str 只是对 String 数据的引用, String是一个所有权型类型, 所以可以直接调用 str 上的方法


    let s = String::from("hello");
    let mut bytes = s.into_bytes();
    println!("{:?}", bytes);
    bytes[0] = 'x' as u8; // 将第一个字符修改为 x
    let s = String::from_utf8(bytes).unwrap();
    println!("{:?}", s);
}
