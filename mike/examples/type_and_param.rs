// 类型和类型参数

// 类型标注
fn type_annotation() {
    println!("***** 类型标注 *****");
    let a: u32 = 10;
    let b = a as u64;
    // let b= a as String; // error
    println!("b = {}", b);
}

// 类型化的好处
// as 显示转换
fn typing_benefits() {
    println!("\n***** 类型化的好处 *****");
    let a = 1.0f32;
    let b = 10 as f32;
    let c = a * b;
    println!("c = {}", c);
}

// 类型作为约束，rust 的 : 在语法层面上就是约束
// 多种类型如何表示？ 类型参数
// 例如 Vec<T>, 支持 Vec<u8>, Vec<u32>, Vec<String>
// T 是类型参数， Vec<T> 整体是泛型（generic type）
fn generic_type() {
    println!("\n***** 泛型 *****");
    #[derive(Debug)]
    struct Point<T> {
        _x: T,
        _y: T,
    }
    // ok
    let wont_work = Point { _x: 5, _y: 4 };
    // let wont_work = Point { _x: 5, _y: 4.0 }; // 两个类型需要一样，编译器对 T 参数进行了推导
    println!("wont_work = {:?}", wont_work);

    // 多个类型参数
    #[derive(Debug)]
    struct Point2<T, U> {
        _x: T,
        _y: U,
    }
    let p = Point2 { _x: 5, _y: 4.0 };
    // turbofish 语法, 显式指定类型 ::<>
    let both_integer = Point2::<u32, u32> { _x: 5, _y: 10 };
    let both_float = Point2::<f32, f32> { _x: 1.0, _y: 4.0 };
    let integer_and_float = Point2::<u32, f32> { _x: 5, _y: 4.0 };
    println!("p = {:?}", p);
    println!("both_integer = {:?}", both_integer);
    println!("both_float = {:?}", both_float);
    println!("integer_and_float = {:?}", integer_and_float);
}

// impl on generic
fn impl_on_generic() {
    println!("\n***** impl on generic *****");
    struct _Point<T> {
        x: T,
        y: T,
    }

    impl<T> _Point<T> {
        fn _play(_n: T) {} // 注意这一行
    }
    // 还可以对具体化类型 做 impl
    impl _Point<u32> {
        // 这里，对具化类型 Point 继续做 impl
        fn _doit() {}
    }
}

// 枚举中的类型参数
fn t_in_enum() {
    println!("\n***** 枚举中的类型参数 *****");
    // 常见的两个枚举：Option<T>用来表示有或无，Result<T, E>用来表示成功或失败
    enum _Option<T> {
        Some(T),
        None,
    }
    enum _Result<T, E> {
        Ok(T),
        Err(E),
    }

    // 更复杂的例子
    struct _Point<T> {
        x: T,
        y: T,
    }
    enum _Aaa<T, U> {
        V1(_Point<T>),
        V2(Vec<U>),
    }
    //实际上，类型参数也是一种复用代码的方式，可以让写出的代码更紧凑
}

// 函数中的类型参数
fn t_in_func() {
    println!("\n***** 函数中的类型参数 *****");
    // struct PointU32 {
    //     x: u32,
    //     y: u32,
    // }

    // struct PointF32 {
    //     x: f32,
    //     y: f32,
    // }

    // fn print_u32(p: PointU32) {
    //     println!("Point {}, {}", p.x, p.y);
    // }

    // fn print_f32(p: PointF32) {
    //     println!("Point {}, {}", p.x, p.y);
    // }

    // let p = PointU32 { x: 10, y: 20 };
    // print_u32(p);

    // let p = PointF32 { x: 10.2, y: 20.4 };
    // print_f32(p);

    struct Point<T> {
        x: T,
        y: T,
    }

    // 这里 T: std::fmt::Display 的意思是要求 T 满足某些条件 / 约束。这里具体来说就是 T 要满足可以被打印的条件
    // rust其实会自动展开成上面这样
    fn print<T: std::fmt::Display>(p: Point<T>) {
        println!("Point {}, {}", p.x, p.y);
    }

    let p = Point { x: 10, y: 20 };
    print(p);

    let p = Point { x: 10.2, y: 20.4 };
    print(p);
}

// 方法中的类型参数
fn t_in_method() {
    println!("\n***** t_in_method *****");
    struct Point<T> {
        x: T,
        _y: T,
    }

    impl<T> Point<T> {
        // 在impl后定义impl block中要用到的类型参数
        fn x(&self) -> &T {
            // 这里，在方法的返回值上使用了这个类型参数
            &self.x
        }
    }

    let p = Point { x: 5, _y: 10 };
    println!("p.x = {}", p.x());
}

fn t_in_method2() {
    println!("\n***** t_in_method2 *****");
    struct Point<X1, Y1> {
        x: X1,
        y: Y1,
    }

    // 这里定义了impl block中可以使用的类型参数X3, Y3，
    impl<X3, Y3> Point<X3, Y3> {
        // 这里单独为mixup方法定义了两个新的类型参数 X2, Y2
        // 于是在mixup方法中，可以使用4个类型参数：X3, Y3, X2, Y2
        fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X3, Y2> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// 类型体系构建方法：struct 结构体，enum 枚举，洋葱结构，type 关键字
// struct 和 enum
fn struct_and_enum() {
    println!("\n***** struct_and_enum *****");
    struct _Point(u32, u32); // 定义点

    struct _Rectangle {
        // 长方形由两个点决定
        p1: _Point,
        p2: _Point,
    }

    struct _Triangle(_Point, _Point, _Point); // 三角形由三个点组成

    struct _Circle(_Point, u32); // 圆由点和半径组成

    enum _Shape {
        // 由枚举把长方形，三角形和圆形聚合在一起
        Rectangle(_Rectangle),
        Triangle(_Triangle),
        Circle(_Circle),
    }

    struct _Axes; // 定义坐标

    struct _Geometry {
        // 几何学由形状和坐标组成
        shape: _Shape,
        axes: _Axes,
    }

    struct _Algebra; // 定义代数

    enum _Level {
        // 定义学校的级别
        Elementary, // 小学
        Secondary,  // 初中
        High,       // 高中
    }

    enum _Course {
        // 数学要学习几何和代数，由枚举来聚合
        Geometry(_Geometry),
        Algebra(_Algebra),
    }

    struct _MathLesson {
        // 定义数学课程，包括数学的科目和级别
        math: _Course,
        level: _Level,
    }
}

// newtype： 结构体常见的封装方法，用单元素的元组结构体
fn newtype() {
    println!("\n***** newtype *****");
    // 它实际就是 Vec 类型的一个新封装，相当于给里面原来那种类型取了一个新名字，
    // 同时也把原类型的属性和方法等屏蔽起来了
    struct _List(Vec<u8>);
}

// 洋葱结构
fn onion() {
    println!("\n***** 洋葱结构 *****");
    use std::collections::HashMap;

    type _AAA = HashMap<String, Vec<u8>>;
    type _BBB = Vec<_AAA>;
    type _CCC = HashMap<String, _BBB>;
    type _DDD = Vec<_CCC>;
    type _EEE = HashMap<String, _DDD>;
}

// newtype_and_struct
fn newtype_and_struct() {
    use std::collections::HashMap;

    struct _AAA(Vec<u8>);
    struct _BBB {
        hashmap: HashMap<String, _AAA>,
    }
    struct _CCC(_BBB);
    type _DDD = Vec<_CCC>;
    type _EEE = HashMap<String, Vec<_DDD>>;

    // 最后，EEE展开就类似下面这样（仅示意，无法编译通过）
    // HashMap<String, Vec<Vec<CCC(BBB {hashmap: HashMap<String, AAA<Vec<u8>>>})>>>
}

// type 关键字
fn type_keyword() {
    use std::collections::HashMap;
    // 处理泛型的情况
    type _MyType<T> = HashMap<String, Vec<HashMap<String, Vec<HashMap<String, Vec<T>>>>>>;

    use std::io::Error;
    pub type _Result<T> = std::result::Result<T, Error>;
}
fn main() {
    type_annotation();
    typing_benefits();
    generic_type();
    impl_on_generic();
    t_in_enum();
    t_in_func();
    t_in_method();
    t_in_method2();
    struct_and_enum();
    newtype();
    onion();
    newtype_and_struct();
    type_keyword();
}
