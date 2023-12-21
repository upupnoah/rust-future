#![allow(dead_code)]
#![allow(unused_variables)]
// 生命周期参数: ' + 名字, 例如 'a, 'b, 'helloworld, 任意
// 'a 用来标识 是否有对外部资源的引用
// 帮助 Rust 的借用检查器(Borrow Checker) 对引用的有效性进行 编译时分析
// 'a 代表某一片代码区间，这片代码区间就是被这种符号标注的引用的有效存在区间
// 对于程序员来说是 语法噪音, 给编译器看的

fn parse_url() {
    println!("\n****** parse_url ******\n");
    #[derive(Debug)]
    struct Url<'a> {
        // 表示结构体 Url 依赖一个外部资源
        // 如果是依赖于不同的字符串资源，可以分开写成不同的生命周期参数 'a、'b 等
        // '名字 可以任意, 小写即可
        protocol: &'a str,
        host: &'a str,
        path: &'a str,
        query: &'a str,
        fragment: &'a str,
    }
    let s = "https://rustcc.cn/article?id=019f9937#title".to_string();
    let _a_url = Url {
        protocol: &s[..5],
        host: &s[8..17],
        path: &s[17..25],
        query: &s[26..37],
        fragment: &s[38..43],
    };
    println!("{:?}", _a_url);

    // 对它做 impl 的时候也需要带上这个参数
    // 'a 的地位与类型参数 T 相似
    impl<'a> Url<'a> {
        fn play() {}
    }

    // 'a 具有传染性, 一旦某个结构体中的某个字段使用了 'a, 那么这个结构体也必须使用 'a
    struct Request<'a> {
        url: Url<'a>,
        body: String,
    }
}

fn lifecycle_param_in_fn() {
    println!("\n****** lifecycle_param_in_fn ******\n");
    // 只要返回引用, 就需要标注生命周期参数
    // fn foo(i: u32, a: &str, b: &str) -> &str {}

    // Rust 分析的时候，不关心这个具体的逻辑，它只看函数签名中的引用之间，有没有可能会发生关联
    fn foo<'a>(i: u32, a: &'a str, b: &'a str) -> &'a str {
        // 第一个 'a 是定义
        // 二,三个 'a: 如果a 和 b 是对同一资源的引用, 那很好理解
        //          : 如果a 和 b 是不同的资源,那么'a的作用是取了一个比较小的代码区间(直到 a 和 b 所引用的资源(较短的那个)释放为止)
        // 第四个: 意味着将 'a 指代的生命周期区间施加到了返回的引用上
        if i == 1 {
            a
        } else {
            b
        }
    }

    // Rust 是按生命周期来分析借用的，而不是靠函数逻辑
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // let s1 = String::from("long string is long");
    // let result;
    // {
    //     let s2 = String::from("xyz");
    //     result = longest(s1.as_str(), s2.as_str()); // 报错:
    // }
    // println!("The longest string is {}", result);
}

/// 类型方法中的引用
fn ref_in_type_method() {
    println!("\n****** ref_in_type_method ******\n");
    struct A {
        foo: String,
    }

    // 比较特别的是，如果返回的值是 Self 本身或本身一部分的引用，就不用手动写 'a 生命周期符号，
    // Rust 会自动帮我们在方法返回值引用的生命周期和 Self 的 scope 之间进行绑定，这是一条默认的规则
    impl A {
        fn play(&self, a: &str, b: &str) -> &str {
            &self.foo
        }
        // 两种写法
        // 1. &self 不变, 返回值是 &'a str
        // 2. 改成&'a self, 返回值是 &str

        // 在类型方法中, &str 表示返回值的生命周期和 self 的生命周期是一样的('1)
        // 这也是为什么play 的返回值是 &str (返回的是 Self 本身或者一部分的引用)
        fn play2<'a>(&self, a: &'a str, b: &str) -> &'a str {
            a
        }
    }
}

fn static_lifecycle() {
    println!("\n****** static_lifecycle ******\n");
    // 'static 生命周期
    // 'static 生命周期是 Rust 中最长的一个生命周期，它的生命周期从程序开始，直到程序结束
    let s: &'static str = "hello world";
}

// 对生命周期的理解
// 为什么放入<> 中?
// 之前只有类型参数 T,U,...  是放入 <> 中的
// ans: 因为生命周期参数跟类型参数一样，也是 generic parameter 的一种，所以放在尖括号里，它俩的地位相同
//      类型参数是空间上的展开（分析），生命周期参数是时间上的展开（分析）

/// 函数可能会被调用多次, 每次调用都有可能传入不同的参数, 也就是说, 每次调用都有可能产生不同的生命周期
fn diff_lifecycle_by_invoke_fn() {
    println!("\n****** diff_lifecycle_by_invoke_fn ******\n");
    fn foo<'a>(a: &'a str, b: &'a str) -> &'a str {
        a
    }
    {
        let s1 = "abc".to_string();
        let s2 = "def".to_string();
        let s3 = foo(&s1, &s2);
        println!("{}", s3);
        // 第一次调用 foo 的 'a 的生命周期到这里结束
    }
    let s4 = "ghk".to_string();
    let s5 = "uvw".to_string();
    let s6 = foo(&s4, &s5);
    println!("{}", s6);
    // 第二次调用 foo 的 'a 的生命周期到这里结束
}

// 关于 'a 的语法噪音
// 不常见的写法, 特别是偏上层的业务代码, 几乎见不到

// 什么时候会写生命周期参数?
// 一般写底层库或对代码做极致性能优化的时候

// 关于 API 的最佳实践
// 在库的 API 设计上，Rust 社区有一条共识：不要向外暴露生命周期参数。这样才能让 API 的使用更简单，并且不会把生命周期符号传染到上层
// 一个反例就是 std 里 Cow 类型的设计，导致现在很少有人会优先选择使用 Cow 类型

// 所有权、借用（引用）、生命周期，这三兄弟是 Rust 中的一套高度耦合的概念，它们共同承担起了 Rust 底层的脏活累活，
// 彻底扫清了最困难的障碍——正确高效地管理内存资源，为 Rust 实现安全编程和高性能编程打下了最坚实的基础
fn main() {
    parse_url();

    lifecycle_param_in_fn();

    ref_in_type_method();

    static_lifecycle();

    diff_lifecycle_by_invoke_fn();
}
