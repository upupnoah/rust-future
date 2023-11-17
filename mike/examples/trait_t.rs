use std::collections::HashMap;

fn t_on_trait() {
    println!("***** t_on_trait *****");
    trait TraitA<T> {}
    // impl 的时候需要指定 for 类型参数
    impl<T> TraitA<T> for i32 {}

    // 也可以在实现的时候将 T 参数具体化
    impl TraitA<u8> for u16 {}

    // 实现的类型上自身也带类型参数，那么情况会更加复杂
    struct Atype<U> {
        _a: U,
    }
    impl<T, U> TraitA<T> for Atype<U> {}

    // 这些类型参数都是可以在 impl 时被约束的，像下面这样：
    struct Atype1<U> {
        _a: U,
    }
    use std::fmt::Debug;
    impl<T, U> TraitA<T> for Atype1<U>
    where
        T: Debug,     // 在 impl 时添加了约束，T 必须实现 Debug
        U: PartialEq, // 在 impl 时添加了约束，U 必须实现 PartialEq
    {
    }
}

fn impl_model() {
    println!("\n***** impl_model *****");
    trait Add<T> {
        type Output; // associated type
        fn add(self, rhs: T) -> Self::Output;
    }

    struct Point {
        x: i32,
        y: i32,
    }

    // 为 Point 实现 Add<Point> 这个 trait
    impl Add<Point> for Point {
        type Output = Self; // 这里的 Self 表示 Point
        fn add(self, rhs: Point) -> Self::Output {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    // 为 Point 实现 Add<i32> 这个 trait
    impl Add<i32> for Point {
        type Output = Self; // 这里的 Self 表示 Point
        fn add(self, rhs: i32) -> Self::Output {
            Point {
                x: self.x + rhs,
                y: self.y + rhs,
            }
        }
    }

    let p1 = Point { x: 1, y: 1 };
    let p2 = Point { x: 2, y: 2 };
    let p3 = p1.add(p2);
    assert_eq!(p3.x, 3);
    assert_eq!(p3.y, 3);

    let p1 = Point { x: 1, y: 1 };
    let delta = 2;
    let p3 = p1.add(delta); // 一个Point实例加一个i32
    assert_eq!(p3.x, 3);
    assert_eq!(p3.y, 3);

    // 总结：
    // Add<Point> 和 Add<i32> 已经是两个不同的 trait 了，因此可以同时为 Point 实现这两个 trait
    // 根据需求，运算后的结果都是 Point
    // Rust 不支持重载，但是使用 trait 可以达到类似的效果
}

fn default_impl_for_trait() {
    println!("\n***** default_impl_for_trait *****");
    // Self 可以用在默认类型位置上
    trait TraitA<T = Self> {
        fn func(_t: T) {} // 没有分号说明是默认实现
    }
    // 这个默认类型为 i32
    trait TraitB<T = i32> {
        fn func2(_t: T) {}
    }
    struct SomeType;

    // 省略类型参数，默认的 T 为 Self
    // 进而 T 就是 SomeType 本身
    impl TraitA for SomeType {
        fn func(_t: SomeType) {}
    }

    // 省略类型参数，默认的 T 为 i32
    impl TraitB for SomeType {
        fn func2(_t: i32) {}
    }

    // 不省略类型参数，明确指定类型参数为 String
    impl TraitA<String> for SomeType {
        fn func(_t: String) {}
    }

    // 不省略类型参数，明确指定类型参数为 String
    impl TraitB<String> for SomeType {
        fn func2(_t: String) {}
    }
}

fn diff_from_constraint_and_define_trait() {
    println!("\n***** diff_from_constraint_and_define_trait *****");
    trait TraitA {
        type Item;
    }
    // 这里，定义结构体类型时，用到了TraitA作为约束
    struct _Foo<T: TraitA<Item = String>> {
        x: T,
    }

    // 定义 trait 时，默认类型参数为 i32
    trait TraitB<T = i32> {}
}

fn demo1() {
    use std::fmt::Debug;

    trait TraitA<T>
    where
        // 定义TraitA的时候，对T作了约束
        T: Debug,
    {
        fn play(&self, _t: T) {}
    }

    struct Atype;

    impl<T> TraitA<T> for Atype where
        T: Debug + PartialEq // 将 TraitA<T> 实现到类型 Atype上时，加强了约束
    {
    }

    // main
    let a = Atype;
    a.play(10u32); // 使用时，通过实例方法传入的参数类型具化 T
}

fn demo2() {
    // 对应上面的 demo1， 对于关联类型来说，如果你在 impl 时不对其具化，就无法编译通过。
    trait Add {
        type ToAdd; // 多定义一个关联类型
        type Output;
        fn add(self, rhs: Self::ToAdd) -> Self::Output;
    }

    struct Point {
        x: i32,
        y: i32,
    }
    impl Add for Point {
        type ToAdd = Point;
        type Output = Point;
        fn add(self, rhs: Point) -> Point {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    // impl Add for Point {
    //     // 这里重复impl 了同一个 Trait，无法编译通过
    //     type ToAdd = i32;
    //     type Output = Point;
    //     fn add(self, rhs: i32) -> Point {
    //         Point {
    //             x: self.x + rhs,
    //             y: self.y + rhs,
    //         }
    //     }
    // }

    // main
    let p1 = Point { x: 1, y: 1 };
    let p2 = Point { x: 2, y: 2 };
    let p3 = p1.add(p2);
    assert_eq!(p3.x, 3);
    assert_eq!(p3.y, 3);
    // let p1 = Point { x: 1, y: 1 };
    // let delta = 2;
    // let p3 = p1.add(delta); // 这句是错的 assert_eq!(p3.x, 3); assert_eq!(p3.y, 3);

    // 总结
    // 这么看起来，好像带类型参数的 trait 功能更强大，
    // 那用这个不就够了？但关联类型也有它的优点，比如关联类型没有类型参数，不存在多引入了一个参数的问题，
    // 而类型参数是具有传染性的，特别是在一个调用层次很深的系统中，增删一个类型参数可能会导致整个项目文件到处都需要改，非常头疼

    // 关联类型适用于比较简单的场景。类型参数适用于更复杂的场景。
    // 在一些场合下，关联类型正好是减少类型参数数量的一种方法。更不要说，有时模型比较简单，
    // 不需要多态特性，这时用关联类型就更简洁，代码可读性更好
}

fn return_multiple_type() {
    // 一个常见的需求，要在一个 Rust 函数中返回可能的多种类型，应该怎么写？
    struct Atype;
    struct Btype;
    struct Ctype;
    // fn doit() -> Atype {
    //     let a = Atype;
    //     a
    // }
    // 方法 1： 使用枚举 enum
    // enum 常用于聚合类型，这些类型之间没有任何关系，使用 enum 可以无脑+强行把他们揉在一起，封闭类型集
    enum _TotalType {
        A(Atype),
        B(Btype),
        C(Ctype),
    }
    fn _doit(i: u32) -> _TotalType {
        if i == 0 {
            let a = Atype;
            _TotalType::A(a) // 在这个分支中返回变体 A
        } else if i == 1 {
            let b = Btype;
            _TotalType::B(b)
        } else {
            let c = Ctype;
            _TotalType::C(c)
        }
    }
    // 方法 2：使用类型参数
    // version1
    // fn doit2<T>() -> T {
    //     let a = Atype;
    //     a // 无法通过编译，因为 T 是调用的时候指定，而不是在定义的时候指定
    // }

    // version2
    // impl Atype {
    //     fn new() -> Atype {
    //         Atype
    //     }
    // }
    // fn doit3<T> () -> T {
    //     T::new() // 还是不行，因为编译器不知道 T 有 new 方法
    // }

    // version3，使用 trait 来定义协议，让 Rust 认识他
    trait TraitA {
        // 关联函数，通过类型直接调用，不需要实例，::<>() 语法
        fn new() -> Self; // TraitA 中定义了 new() 函数
        fn play(&self) {} // TraitA 中定义了 play() 函数
    }
    impl TraitA for Atype {
        fn new() -> Atype {
            Atype
        }
    }
    impl TraitA for Btype {
        fn new() -> Btype {
            Btype
        }
        fn play(&self) {
            println!("Btype play")
        }
    }
    impl TraitA for Ctype {
        fn new() -> Ctype {
            Ctype
        }
    }
    // T: TraitA 是一个约束，表示 T 必须实现 TraitA 这个 trait
    fn doit3<T: TraitA>() -> T {
        T::new()
    }
    // main
    // 通过 :: 指定具体的类型
    // <> 中明确了 T 的类型
    let _a: Atype = doit3::<Atype>();
    let _b = doit3::<Btype>();
    let _c: Ctype = doit3();

    // version 4， 更加优雅的方式：特殊语法： impl trait
    // 只要返回的结果实现了 TraitA 这个 trait，就可以返回
    fn doit4() -> impl TraitA {
        // 这个返回值是固定的（只能返回一个），会自动推导， 内部使用 if else + return 都会报错
        // let a = Atype;
        // a
        let b = Btype;
        b
        // let c = Ctype;
        // c
    }
    // main
    let _b = doit4();
    _b.play(); // 可以直接调用 TraitA 中定义的 play() 函数
}

fn trait_object() {
    // trait object
    // dyn TraitName 本身就是一种类型
    struct Atype;
    struct Btype;
    struct Ctype;
    trait TraitA {}
    impl TraitA for Atype {}
    impl TraitA for Btype {}
    impl TraitA for Ctype {}

    // Box<T> 的作用是可以保证获得里面值的所有权，必要的时候会进行内存的复制，比如把栈上的值复制到堆中去。
    // 一旦值到了堆中，就很容易掌握到它的所有权
    fn doit(i: u32) -> Box<dyn TraitA> {
        // 返回类型改为 dyn TraitA，而不是 impl TraitA
        if i == 0 {
            let a = Atype;
            Box::new(a) // 在这个分支中返回变体 A
        } else if i == 1 {
            let b = Btype;
            Box::new(b)
        } else {
            let c = Ctype;
            Box::new(c)
        }
    }
    // main
    let _a = doit(0);
}

// 利用 trait object 传参
// trait object:
// impl TraitName
// dyn TraitName

// 原理：
// impl TraitName：会在编译器静态展开，生成具体的类型
// dyn TraitName：而 dyn trait 的版本不会在编译期间做任何展开，
//      dyn TraitA 自己就是一个类型，这个类型相当于一个代理类型，
//      用于在运行时代理相关类型及调用对应方法。既然是代理，也就是调用方法的时候需要多跳转一次，
//      从性能上来说，当然要比在编译期直接展开一步到位调用对应函数要慢一点

// 静态展开也有问题，就是会使编译出来的内容体积增大，而 dyn trait 就不会
// 另外，impl trait 和 dyn trait 都是消除类型参数的办法

// enum：封闭类型集， 拓展麻烦，适用于库开发
// trait object：开放类型集， 拓展简单，适用于应用开发
//      相比于 impl traitName， dyn traitName 有更多的灵活性，适用于更多场景
fn use_trait_object_to_pass_parameter() {
    // impl TraitName
    println!("\n***** impl TraitName *****");
    struct Atype;
    struct Btype;
    struct Ctype;
    trait TraitA {}
    impl TraitA for Atype {}
    impl TraitA for Btype {}
    impl TraitA for Ctype {}

    fn doit(_x: impl TraitA) {}
    // 等价于
    // fn doit<T: TraitA>(x: T) {}

    // main
    let a = Atype;
    doit(a);
    let b = Btype;
    doit(b);
    let c = Ctype;
    doit(c);

    // dyn TraitName
    println!("\n***** dyn TraitName *****");
    fn doit2(_x: &dyn TraitA) {} // 这里用了引用

    // main
    let a1 = Atype;
    doit2(&a1);
    let b1 = Btype;
    doit2(&b1);
    let c1 = Ctype;
    doit2(&c1);
}

// 利用 trait obj 将不同的类型装进集合里
fn use_trait_obj_put_difftype_in_set() {
    struct Atype;
    struct Btype;
    struct Ctype;

    trait TraitA {}

    impl TraitA for Atype {}
    impl TraitA for Btype {}
    impl TraitA for Ctype {}

    let a = Atype;
    let b = Btype;
    let c = Ctype;

    // vec 要求所有元素类型相同，所以这里会报错
    // let v = vec![a, b, c];

    // 使用 trait object 可以绕过这个限制
    let _v: Vec<&dyn TraitA> = vec![&a, &b, &c];
    // 使用&dyn TraitA，绕过 HashMap 的限制
    let mut map: HashMap<&str, &dyn TraitA> = HashMap::new();
    map.insert("a", &a);
    map.insert("b", &b);
    map.insert("c", &c);
}

// 可以作为 trait object 的 trait（只有安全的 trait 才可以）
// 安全的 Trait
// trait TraitA {
//     fn foo(&self) {}
//     fn foo_mut(&mut self) {}
//     fn foo_box(self: Box<Self>) {}
// }

// 不安全的 Trait
// trait NotObjectSafe {
//     const CONST: i32 = 1;  // 不能包含关联常量

//     fn foo() {}  // 不能包含这样的关联函数
//     fn selfin(self); // 不能将Self所有权传入
//     fn returns(&self) -> Self; // 不能返回Self
//     fn typed<T>(&self, x: T) {} // 方法中不能有类型参数
// }

// 规则比较复杂，但是可以看几种简单的场景
// 1. 不要在 trait 里面定义构造函数，比如 new 这种返回 Self 的关联函数。
// 你可以发现，确实在整个 Rust 生态中都没有将构造函数定义在 trait 中的习惯

// 2. trait 里面尽量定义传引用 &self 或 &mut self 的方法，而不要定义传值 self 的方法

fn main() {
    // 带类型参数的 trait 及 trait object

    // trait 上带类型参数
    t_on_trait();

    // example for impl
    impl_model();

    // trait 类型参数的默认实现
    default_impl_for_trait();

    // 具化关联类型在 应用约束 与 定义 trait 的不同
    diff_from_constraint_and_define_trait();

    // trait 中的类型参数与关联类型的区别
    // 1. 类型参数可以在 impl 类型的时候具化，也可以延迟到使用的时候具化。而关联类型在被 impl 时就必须具化
    demo1();
    // 2. 由于类型参数和 trait 名一起组成了完整的 trait 名字，不同的具化类型会构成不同的 trait，
    // 所以看起来同一个定义可以在目标类型上实现“多次”。 而关联类型没有这个作用
    demo2();

    return_multiple_type();

    trait_object();

    // 利用 trait object 传参
    use_trait_object_to_pass_parameter();

    // 利用 trait obj 将不同的类型装进集合里
    use_trait_obj_put_difftype_in_set();
}
