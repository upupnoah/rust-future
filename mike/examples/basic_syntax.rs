fn main() {
    // let 语句
    println!("***** let *****");
    // 变量 a 的类型是 u32, 也就是无符号 32 位整数，赋值为 1
    let _a: u32 = 1; // Rust 保证你定义的变量在第一次使用之前一定被初始化过

    // 字符串 String
    let _hello = String::from("你好");
    // 你可能想把"你"字取出来，但实际上这样是错误的
    // let a = hello[0];

    // String 转义
    println!("\n***** String 转义 *****");

    // 将""号进行转义
    let byte_escape = "I'm saying \"Hello\"";
    println!("{}", byte_escape);

    // 分两行打印
    let byte_escape = "I'm saying \n 你好";
    println!("{}", byte_escape);

    // Windows下的换行符
    let byte_escape = "I'm saying \r\n 你好";
    println!("{}", byte_escape);

    // 打印出 \ 本身
    let byte_escape = "I'm saying \\ Ok";
    println!("{}", byte_escape);

    // 强行在字符串后面加个0，与C语言的字符串一致。
    let byte_escape = "I'm saying hello.\0";
    println!("{}", byte_escape);

    // 使用 \x 输入等值的ASCII字符（最高7位）
    let byte_escape = "I'm saying hello \x7f";
    println!("{}", byte_escape);

    // 使用 \u{} 输入等值的Unicode字符（最高24位）
    let byte_escape = "I'm saying hello \u{0065}";
    println!("{}", byte_escape);

    // 禁止转义的字符串字面量
    println!("\n***** 禁止转义的字符串字面量 *****");
    // 字符串字面量前面加r，表示不转义
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 这个字面量必须使用r##这种形式，因为我们希望在字符串字面量里面保留""
    let quotes = r#"And then I said: "There is no escape!"#;
    println!("{}", quotes);

    // 如果遇到字面量里面有#号的情况，可以在r后面，加任意多的前后配对的#号，
    // 只要能帮助Rust编译器识别就行
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);

    // 字节串
    println!("\n***** 字节串 *****");

    // 字节串的类型是字节的数组，而不是字符串了
    let bytestring: &[u8; 21] = b"this is a byte string";
    println!("A byte string: {:?}", bytestring);

    // 可以看看下面这串打印出什么
    let escaped = b"\x52\x75\x73\x74 as bytes";
    println!("Some escaped bytes: {:?}", escaped);

    // 字节串与原始字面量结合使用
    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);

    // 数组
    println!("\n***** 数组 *****");
    // [type; size]
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a[2] = {}", a[2]);
    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // 动态数组 Vector
    println!("\n***** 动态数组 Vector *****");

    let _v: Vec<i32> = Vec::new();
    let _v = vec![1, 2, 3];

    let mut _v = Vec::new();
    _v.push(5);
    _v.push(6);
    _v.push(7);
    _v.push(8);

    let s1 = String::from("superman 1");
    let s2 = String::from("superman 2");
    let s3 = String::from("superman 3");
    let s4 = String::from("superman 4");

    let v = vec![s1, s2, s3, s4];

    println!("{:?}", v[0]);

    // HashMap
    println!("\n***** HashMap *****");

    use std::collections::HashMap;
    
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);
}
