// use default trait
fn default_trait() {
    println!("***** use default trait *****");
    struct Color(u8, u8, u8);
    impl Default for Color {
        // 默认颜色是黑色（0，0，0）
        // 内敛函数，可以通过Color::default()调用
        fn default() -> Self {
            Color(0, 0, 0)
        }
    }
    let _color = Color::default();
    // 或
    let _color: Color = Default::default();

    fn _paint(color: Option<Color>) {
        // 如果没有颜色，就用默认颜色
        let _color = color.unwrap_or_default();
        // ...
    }
    // 由于default()是在trait中定义的关联函数，因此可方便的由类型参数调用
    fn _guarantee_length<T: Default>(mut vec: Vec<T>, min_len: usize) -> Vec<T> {
        for _ in 0..min_len.saturating_sub(vec.len()) {
            vec.push(T::default()); // 这里用了 T::default() 这种形式
        }
        vec
    }
}

fn default_on_struct() {
    // 结构体部分更新是 default 的作用
    println!("\n***** 结构体部分更新是 default 的作用 *****");

    // 派生宏， 自动实现了 Default trait
    // 为什么能够自动实现? 因为 Color 的所有字段都实现了 Default trait
    #[derive(Default)]
    struct Color {
        _r: u8,
        _g: u8,
        _b: u8,
    }
    impl Color {
        fn _new(_r: u8, _g: u8, _b: u8) -> Self {
            Color { _r, _g, _b }
        }
    }
    impl Color {
        fn _red(_r: u8) -> Self {
            Color {
                _r,
                ..Color::default() // 从 Color 中拷贝其他字段的默认值
            }
        }
        fn _green(_g: u8) -> Self {
            Color {
                _g,
                ..Color::default() // 注意这一句
            }
        }
        fn _blue(_b: u8) -> Self {
            Color {
                _b,
                ..Color::default() // 注意这一句
            }
        }
    }
}

fn display_trait() {
    // Dispaly trait
    println!("\n***** Dispaly trait *****");
    use std::fmt;

    #[derive(Default)]
    struct Point {
        x: i32,
        y: i32,
    }

    // 为 Point 实现 Display
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y) // write!宏向stdout写入
        }
    }
    println!("origin: {}", Point::default());
    // 打印出 "origin: (0, 0)"
    // 在 format! 中用 "{}" 将类型表示/转换为 String
    let stringified = format!("{}", Point::default());
    assert_eq!("(0, 0)", stringified); // ✅
}

fn to_string_trait() {
    // ToString trait 标准库做了总实现
    // 只要实现了 Display trait， 就可以自动实现 ToString trait
    println!("\n***** ToString trait *****");
    use std::fmt;
    #[derive(Default)] 
    struct Point {
        x: i32,
        y: i32,
    }
    // 为Point实现 Display
    impl fmt::Display for Point {
        // 实现唯一的fmt方法，这里定义用户自定义的格式
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y) // write!宏向stdout写入
        }
    }

    println!("origin: {}", Point::default());
    // 打印出 "origin: (0, 0)"
    // 在 format! 中用 "{}" 将类型表示/转换为 String
    let stringified = format!("{}", Point::default());
    assert_eq!("(0, 0)", stringified); // ✅

    // 把一个符合条件的类型实例 转换成字符串有两种常用方法
    // let s = format!("{}", obj);
    // let s = obj.to_string();
}

fn debug_trait() {
    println!("\n***** Debug trait *****");
    // {:?}
    // {:#?} 更加美观
    // Rust 的类型能够自动被 derive 的条件是，它里面的每个元素都能被 derive
    // 比如下方这个结构体，它里面的每个元素类型都实现了 Debug trait， 因此可以自动 derive Debug
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    let point = Point { x: 0, y: 0 };
    println!("origin: {:?} {:?}", point.x, point.y);
}

fn partialeq_and_eq_trait() {
    println!("\n***** PartialEq and Eq trait *****");
    // 特殊:浮点数实现了 PartialEq， 但是没有实现 Eq
    // PartialEq: = and !=
    // Eq: == and != and 自反性(a == a)

    // Eq 更加完备， 但是 PartialEq 更加常用
    #[derive(PartialEq, Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    fn _example_assert(p1: Point, p2: Point) {
        assert_eq!(p1, p2);
    }
    fn _exmaple_compare_collections<T: PartialEq>(vec1: Vec<T>, vec2: Vec<T>) {
        if vec1 == vec2 {
            // some code
        } else {
            // other code
        }
    }
}

fn partialord_and_ord() {
    println!("\n***** PartialOrd and Ord trait *****");
    // 特殊:浮点数实现了 PartialOrd， 但是没有实现 Ord
    // 如果我们为一个类型实现了 Ord，那么对那个类型的所有值，我们可以做出一个严格的总排序
    #[derive(PartialEq, PartialOrd)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[derive(PartialEq, PartialOrd)]
    enum Stoplight {
        _Red,
        _Yellow,
        _Green,
    }

    // 由于 Ord 严格的顺序性，如果一个类型实现了 Ord，那么这个类型可以被用作 BTreeMap 或 BTreeSet 的 key
    //   BTreeMap、BTreeSet：相对于 HashMap 和 HashSet，是两种可排序结构
    use std::collections::BTreeSet;

    #[derive(Ord, PartialOrd, PartialEq, Eq)]
    struct Point2 {
        x: i32,
        y: i32,
    }
    fn _example_btreeset() {
        let mut points = BTreeSet::new();
        points.insert(Point2 { x: 0, y: 0 }); // 作 key 值插入
    }
    // 实现了 Ord trait 的类型的集合， 可调用 .sort() 排序方法
    fn _example_sort<T: Ord>(mut sortable: Vec<T>) -> Vec<T> {
        sortable.sort();
        sortable
    }
}

// 运算符重载
// Add trait
fn operator_overloading() {
    #[derive(Copy, Clone)]
    struct Point {
        x: i32,
        y: i32,
    }
    use std::ops::Add;
    // 为 Point 类型实现 Add trait， 这样两个 Point 实例就可以直接相加
    // 为 &Point 实现 Add trait，这样相加不会消耗所有权
    impl Add for Point {
        type Output = Point;
        fn add(self, rhs: Point) -> Point {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    // let p3 = &p1 + &p2; // 为 &Point 实现 Add trait
    let p3 = p1 + p2; // 这里我们为 Point 派生了 Copy trait，因此 p1 和 p2 的所有权不会被转移
    assert_eq!(p3.x, p1.x + p2.x);
    assert_eq!(p3.y, p1.y + p2.y);
}

fn clone_trait() {
    // Clone trait
    // 给目标类型提供 clone() 方法
    println!("\n***** Clone trait *****");
    #[derive(Clone, Debug)]
    struct Point {
        x: u32,
        y: u32,
    }
    let p1 = Point { x: 1, y: 2 };
    let _p2 = p1.clone();
    println!("{:#?}, {:#?}", p1.x, p1.y);

    // 有两种情况
    // 1. 已经拿到实例的所有权，clone 一份生成一个新的所有权并被局部变量所持有
    // 2. 只拿到一个实例的引用，想拿到它的所有权，如果这个类型实现了 Clone trait， 那么就可以 clone 一份拿到这个所有权

    // clone() 是对象的深度拷贝，可能会有比较大的额外负载，但是就大多数情况来说其实还好
    // 不要担心在 Rust 中使用 clone()，先把程序功能跑通最重要

    // 浅拷贝是按值拷贝一块连续的内存，只复制一层，不会去深究这个值里面是否有到其它内存资源的引用。
    // 与之相对，深拷贝就会把这些引用对象递归全部拷贝
}

// copy 是浅拷贝
fn copy_trait() {
    // Clone 是 Copy 的父Trait，意味着实现了 Clone 的类型，也必须实现 Copy
    // trait Copy: Clone {}
    // 不能为自定义类型 impl Copy，但是可以通过 derive Copy 来自动实现

    //不能为 Atype 派生 Copy, 因为它的成员变量 Vec<u32> 是管理堆内存的动态数组
    // 如果可以实现 Copy, 那么会出现多个实例同时管理同一块堆内存的情况，这是不允许的
    #[derive(Clone, Debug)]
    struct Atype {
        _num: u32,
        _a_vec: Vec<u32>, // 动态数组资源在堆内存中
    }

    // Clone
    #[derive(Clone)]
    struct Point {
        _x: u32,
        _y: u32,
    }
    let a = Point { _x: 10, _y: 10 };
    let _b = a; // a 的所有权转移到 b

    // Copy
    #[derive(Copy, Clone)]
    struct Point2 {
        _x: u32,
        _y: u32,
    }
    let a = Point2 { _x: 10, _y: 10 };
    let _b = a; // 这里发生了复制，a在后续可以继续使用
    let _c = a; // 这里又复制了一份，这下有3份了
}

fn to_owned_trait() {
    // ToOwned 相当于是 Clone 更宽泛的版本。ToOwned 给类型提供了一个 to_owned() 方法，可以将引用转换为所有权实例
    println!("\n***** ToOwned trait *****");
    let a = "123456";
    let _s = a.to_owned();
}

fn deref_trait() {
    println!("\n***** Deref trait *****");
    // Deref trait 可以用来把一种类型转换成另一种类型，但是要在引用符号 &、点号操作符 . 或其他智能指针的触发下才会产生转换
    // 比如标准库里最常见的 &String 可以自动转换到 &str, 就是因为 String 类型实现了 Deref trait
}

fn drop_trait() {
    println!("\n***** Drop trait *****");
    // Drop trait 用于给类型做自定义垃圾清理（回收）
    // 实现了这个 trait 的类型的实例在走出作用域的时候，触发调用 drop() 方法，这个调用发生在这个实例被销毁之前
    struct A;
    impl Drop for A {
        fn drop(&mut self) {
            // 可以尝试在这里打印点东西看看什么时候调用
        }
    }
    // 一般来说，我们不需要为自己的类型实现这个 trait，除非遇到特殊情况，
    // 比如我们要调用外部的 C 库函数，然后在 C 那边分配了资源，由 C 库里的函数负责释放，
    // 这个时候我们就要在 Rust 的包装类型（对 C 库中类型的包装）上实现 Drop，并调用那个 C 库中释放资源的函数
}

// 闭包相关 trait
fn closure_trait() {
    // FnOnce: 获取上下文环境的所有权
    // FnMut: 只获取了上下文环境的 &mut 引用
    // Fn: 只获取了上下文环境的 & 引用
    // 在 Rust 中，并不把这个闭包的类型处理成 fn 这种函数指针类型，而是有单独的类型定义

    // 闭包 demo
    let range = 0..10;

    // FnOnce 代表的闭包类型只能被调用一次
    let get_range_count = || range.count();
    assert_eq!(get_range_count(), 10); // 第二次调用 get_range_count(); // ❌ 所有权转移了

    // FnMut 代表的闭包类型可以被调用多次，并且能够修改上下文环境(值),副作用: 导致错误或者不可预测的行为
    let nums = vec![0, 4, 2, 8, 10, 7, 15, 18, 13];
    let mut min = i32::MIN;
    let ascending = nums
        .into_iter()
        .filter(|&n| {
            if n <= min {
                false
            } else {
                min = n; // 这里修改了环境变量min的值!!!! 内部有修改就说明是 FnMut
                true
            }
        })
        .collect::<Vec<_>>();
    assert_eq!(vec![0, 4, 8, 10, 15, 18], ascending); // ✅

    // Fn 代表的这类闭包能被调用多次，但是对上下文环境变量没有副作用
    let nums = vec![0, 4, 2, 8, 10, 7, 15, 18, 13];
    let min = 9;
    let greater_than_9 = nums.into_iter().filter(|&n| n > min).collect::<Vec<_>>();
    assert_eq!(vec![10, 15, 18, 13], greater_than_9); // ✅

    // 另外，fn 这种函数指针，用在不需要捕获上下文环境变量的场景
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    let mut fn_ptr: fn(i32) -> i32 = add_one; // 注意这里的类型定义
    assert_eq!(fn_ptr(1), 2); // ✅

    // 如果一个闭包没有捕捉环境变量，它可以通过类型转换转成 fn 类型
    fn_ptr = |x| x + 1; // same as add_one
    assert_eq!(fn_ptr(1), 2); // ✅
}

fn fromt_and_intot() {
    println!("\n***** From<T> and Into<T> trait *****");
    // 我们看 Rust 标准库中的两个关联的 trait From<T> 和 Into<T>，它们用于类型转换
    // From 可以把类型 T 转为自己，而Into 可以把自己转为类型 T
    // Rust 只允许我们实现 From，因为实现了 From 后，自动就实现了 Into (互逆)

    // demo1
    fn _function<T>(_t: T)
    where
        // 下面这两种约束是等价的
        T: From<i32>,
        i32: Into<T>,
    {
        // 等价
        let _example = T::from(0);
        let _example: T = 0.into();
    }

    // demo2
    struct Point {
        _x: i32,
        _y: i32,
    }
    impl From<(i32, i32)> for Point {
        // 实现从(i32, i32)到Point的转换
        fn from((_x, _y): (i32, i32)) -> Self {
            Point { _x, _y }
        }
    }
    impl From<[i32; 2]> for Point {
        // 实现从[i32; 2]到Point的转换
        fn from([_x, _y]: [i32; 2]) -> Self {
            Point { _x, _y }
        }
    }
    fn _example() {
        // 使用from()转换不同类型
        let _origin = Point::from((0, 0));
        let _origin = Point::from([0, 0]);
        // 使用into()转换不同类型
        let _origin: Point = (0, 0).into();
        let _origin: Point = [0, 0].into();
    }

    // Into 有个常用的比 From 更自然的场景是，如果你已经拿到了一个变量，想把它变成具有所有权的值，Into 写起来更顺手
    // 因为 into() 是方法，而 from() 是关联函数
}

fn tryfrom_tryinto() {
    println!("\n***** TryFrom<T> and TryInto<T> trait *****");
    // 他们是 From<T> 和 Into<T> 可失败版本
    // 如果你认为转换可能会出现失败的情况，就选择这两个 trait 来实现

    // 返回的是 Result, 需要进行错误处理
    // trait TryFrom<T> {
    //     type Error;
    //     fn try_from(value: T) -> Result<Self, Self::Error>;
    // }

    // trait TryInto<T> {
    //     type Error;
    //     fn try_into(self) -> Result<T, Self::Error>;
    // }
}

fn fromstr_trait() {
    println!("\n***** FromStr trait *****");
    // 从字符串类型转换到自身
    // trait FromStr {
    //     type Err;
    //     fn from_str(s: &str) -> Result<Self, Self::Err>;
    // }
    use std::str::FromStr;

    fn example<T: FromStr>(s: &str) {
        // 下面4种表达等价
        let _t: Result<T, _> = FromStr::from_str(s);
        let _t = T::from_str(s);
        let _t: Result<T, _> = s.parse();
        let _t = s.parse::<T>(); // 最常用的写法
    }
    example::<i32>("123");
}

fn asref_trait() {
    println!("\n***** AsRef trait *****");
    // trait AsRef<T> {
    //     fn as_ref(&self) -> &T;
    // }

    // deref: 隐式调用
    // as_ref(): 显式调用, 所以代码会更清晰，出错的机会也会更少

    // 使用 &str 作为参数可以接收下面两种类型
    //  - &str
    //  - &String
    fn _takes_str(_s: &str) {
        // use &str
    }
    // 使用 AsRef<str> 作为参数可以接受下面三种类型
    //  - &str
    //  - &String
    //  - String
    fn _takes_asref_str<S: AsRef<str>>(s: S) {
        let _s = s.as_ref();
        // use &str
    }
    fn _example(slice: &str, borrow: &String, owned: String) {
        _takes_str(slice);
        _takes_str(borrow);
        // takes_str(owned); // ❌ excepted &str, found String
        _takes_asref_str(slice);
        _takes_asref_str(borrow);
        _takes_asref_str(owned); // ✅
    }

    // 你可以把 Deref 看成是隐式化（或自动化）+ 弱化版本的 AsRef<T>。
}
fn main() {
    // use default trait
    default_trait();

    // default on struct
    default_on_struct();

    // Dispaly trait
    display_trait();

    // ToSTring trait
    to_string_trait();

    // Debug trait
    debug_trait();

    // PartialEq and Eq trait
    partialeq_and_eq_trait();

    // PartialOrd and Ord trait
    partialord_and_ord();

    // operator overloading
    operator_overloading();

    // 显式地留下足迹，是 Rust 语言设计重要的哲学之一
    // Rust 鼓励优先使用 Clone 而不鼓励使用 Copy

    // Clone trait
    clone_trait();

    // Copy trait
    copy_trait();

    // ToOwned trait
    to_owned_trait();

    // Deref trait
    deref_trait();

    // Drop trait
    drop_trait();

    // closure trait
    closure_trait();

    // From<T> and Into<T> trait
    fromt_and_intot();

    // TryFrom<T> and TryInto<T> trait
    tryfrom_tryinto();

    // FromStr trait
    fromstr_trait();

    // AsRef trait
    asref_trait();
}
