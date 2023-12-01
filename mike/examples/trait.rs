fn basic_trait() {
    println!("***** basic_trait *****");
    // trait TraitA {}

    // struct Atype;

    // impl TraitA for Atype {}

    trait Sport {
        fn play(&self) {} // 注意这里一对花括号，就是trait的关联函数的默认实现
        fn play_mut(&mut self) {}
        fn play_own(self); // 注意这里是以分号结尾，就表示没有默认实现
        fn play_some() -> Self; // 关联函数：第一次参数不是 self，他被实现在具体类型上
    }

    struct Football;

    // impl Sport for Football {
    //     fn play(&self) {} // 注意函数后面的花括号，表示实现
    //     fn play_mut(&mut self) {}
    //     fn play_own(self) {}
    //     fn play_some() -> Self {
    //         Self
    //     }
    // }

    impl Sport for Football {
        // 因为上面的 trait 中有默认实现，因此这里可以不用实现
        fn play_own(self) {}
        fn play_some() -> Self {
            Self
        }
    }

    // 使用 trait
    let mut f = Football;
    f.play(); // 方法在实例上调用
    f.play_mut();
    f.play_own();
    let _g = Football::play_some(); // 关联函数要在类型上调用
    let _g = <Football as Sport>::play_some(); // 注意这样也是可以的
}

// 关联类型
fn associated_type() {
    println!("\n***** associated_type *****");
    pub trait Sport {
        type SportType; // 关联类型, 占位符，具体类型由实现者指定
        fn play(&self, st: SportType);
    }

    struct Football;

    #[derive(Debug)]
    pub enum SportType {
        Land,
        _Water,
    }

    impl Sport for Football {
        type SportType = SportType; // 这里故意取相同的名字，不同的名字也是可以的
        fn play(&self, st: SportType) {
            println!("{:?}", st)
        } // 方法中用到了关联类型
    }

    let f = Football;
    f.play(SportType::Land)
}

// 在 T 上使用关联类型
fn use_associated_type_in_t() {
    // 先有 struct，然后再定义 trait， 再给某个 struct 实现 trait
    println!("\n***** use_associated_type_in_t *****");
    pub trait Iterator {
        type Item;

        // Self::Item 是 <Self as Iterator>::Item 的简写
        // 一般来说，如果一个类型参数被 TraitA 约束，而 TraitA 里有关联类型 MyType，那么可以用 T::Mytype 这种形式来表示路由到这个关联类型
        fn next(&mut self) -> Option<Self::Item>;
    }

    trait TraitA {
        type MyType;
    }

    fn doit<T: TraitA>(_a: T::MyType) {} // 这里在函数中使用了关联类型

    struct TypeA;
    impl TraitA for TypeA {
        type MyType = String; // 具体关联类型为 String
    }
    doit::<TypeA>("abc".to_string()); // T 具化为 TypeA
}

// 在约束中具化关联类型
fn associated_type_in_constraint() {
    println!("\n***** associated_type_in_constraint *****");
    trait TraitA {
        type Item;
    }

    // 限制必须实现了 TraitA，而且它的关联类型必须是 String 才能代入这个 T
    struct Foo<T: TraitA<Item = String>> {
        // 这里在约束表达式中对关联类型做了具化
        _x: T,
    }
    struct A;
    impl TraitA for A {
        type Item = String;
    }

    let _a = Foo { _x: A };
}

// 对关联类型的约束
fn constraint_for_associated_type() {
    println!("\n***** constraint_for_associated_type *****");
    use std::fmt::Debug;

    trait TraitA {
        type Item: Debug; // 这里对关联类型添加了 Debug 约束
    }

    #[derive(Debug)]
    struct A;

    struct B;

    impl TraitA for B {
        type Item = A; // 这里这个类型 A 已满足 Debug 约束
    }
}

// 加强对关联类型的约束
fn strengthen_constraints_on_associated_types() {
    println!("\n***** strengthen_constraints_on_associated_types *****");
    use std::fmt::Debug;

    trait TraitA {
        type Item: Debug; // 这里对关联类型添加了 Debug 约束
    }

    #[derive(Debug)]
    struct A;

    struct B;

    impl TraitA for B {
        type Item = A; // 这里这个类型 A 已满足 Debug 约束
    }

    // 表示只有实现过 TraitA 且其关联类型 Item 的具化版必须满足 Debug 和 PartialEq 的约束
    fn _doit<T>()
    where
        T: TraitA,                  // 使用 where 语句将 T 的约束表达放在后面来
        T::Item: Debug + PartialEq, // 注意这一句，直接对TraitA的关联类型Item添加了更多一个约束 PartialEq {}
    {
    }
}

// 关联常量
fn associated_constant() {
    println!("\n***** associated_constant *****");
    trait TraitA {
        const LEN: u32 = 10;
    }

    struct A;
    impl TraitA for A {
        const LEN: u32 = 12;
    }
    println!("{:?}", A::LEN);
    println!("{:?}", <A as TraitA>::LEN);
}

// where
fn where_in_trait() {
    println!("\n***** where_in_trait *****");
    // 这样很丑
    // fn doit<T: TraitA + TraitB + TraitC + TraitD + TraitE>(t: T) -> i32 {}

    // 可以写成这样
    // fn doit<T>(t: T) -> i32
    // where
    //     T: TraitA + TraitB + TraitC + TraitD + TraitE,
    // {
    // }
}

// 约束依赖
fn constraint_dependence() {
    println!("\n***** constraint_dependence *****");

    // 如果某种类型要实现 TraitA，那么它也要同时实现 TraitB
    // trait TraitB {
    //     type Item;
    // }
    // trait TraitA: TraitB {}

    // this code is equal to the following code
    trait Shape {
        fn area(&self) -> f64;
    }
    trait Circle: Shape {
        fn radius(&self) -> f64;
    }
    // trait Shape {
    //     fn area(&self) -> f64;
    // }
    // trait Circle
    // where
    //     Self: Shape,
    // {
    //     fn radius(&self) -> f64;
    // }

    // T: Circle
    //     实际上表示：
    // T: Circle + Shape
    struct A;
    struct _B;
    impl Shape for A {
        fn area(&self) -> f64 {
            0.0
        }
    }
    impl Circle for A {
        fn radius(&self) -> f64 {
            0.0
        }
    }
    // impl Circle for B {}

    // 一个 trait 可以继承（依赖）多个 trait
    // 约束之间是完全平等的，理解这一点非常重要
    // trait TraitA: TraitB + TraitC {}
}

// 约束中同名方法的访问
fn access_to_methods_with_the_same_name_in_constraints() {
    println!("\n***** access_to_methods_with_the_same_name_in_constraints *****");
    trait Shape {
        fn play(&self) {
            println!("1");
        }
    }
    trait Circle: Shape {
        fn play(&self) {
            println!("2");
        }
    }
    struct A;
    impl Shape for A {}
    impl Circle for A {}
    impl A {
        fn play(&self) {
            // 又直接在 A 上实现了 play() 方法
            println!("3");
        }
    }

    // 完全限定语法
    let a = A;
    a.play(); // 调用类型 A 上实现的 play()方法
    <A as Circle>::play(&a); // 调用 trait Circle 上实现的 play() 方法
    <A as Shape>::play(&a); // 调用 trait Shape 上实现的 play() 方法
}

// 用 trait 实现能力配置
// Rust 如何检查某个实例有没有某个方法
// 1. 检查有没有直接在这个类型上实现这个方法
// 2. 检查有没有在这个类型上实现某个 trait，trait 中有这个方法

// Rust 在这里采用了一种惰性的机制，由开发者指定在当前的 mod 或 scope 中使用哪套或哪几套能力
// 因此，对应地需要开发者手动地将要用到的 trait 引入当前 scope

mod module_a {
    pub trait Shape {
        fn play(&self) {
            println!("1");
        }
    }
    pub struct A;
    impl Shape for A {}
}

// Rust 的 trait 需要引入当前 scope 才能使用的方式可以看作是能力配置（Capability Configuration）机制
mod module_b {
    // use super::module_a::Shape; // 引入这个 trait
    // use super::module_a::A; // 这里只引入了另一个模块中的类型
    use super::module_a::{Shape, A}; // 引入 Shape trait
    fn _doit() {
        let a: A = A;
        a.play();
    }
}

// 约束可按需配置
fn constraint_as_needed() {
    println!("\n***** constraint_as_needed *****");
    trait TraitA {}
    trait TraitB {}
    trait TraitC {}

    struct A;
    struct B;
    struct C;

    impl TraitA for A {}
    impl TraitB for A {}
    impl TraitC for A {} // 对类型A实现了TraitA, TraitB, TraitC
    impl TraitB for B {}
    impl TraitC for B {} // 对类型B实现了TraitB, TraitC
    impl TraitC for C {} // 对类型C实现了TraitC

    // 7个版本的doit() 函数
    fn doit1<T: TraitA + TraitB + TraitC>(_t: T) {}
    fn doit2<T: TraitA + TraitB>(_t: T) {}
    fn doit3<T: TraitA + TraitC>(_t: T) {}
    fn doit4<T: TraitB + TraitC>(_t: T) {}
    fn doit5<T: TraitA>(_t: T) {}
    fn doit6<T: TraitB>(_t: T) {}
    fn doit7<T: TraitC>(_t: T) {}

    // 观察 A 实现了哪些 trait，看是否匹配，只能多，不能少
    doit1(A);
    doit2(A);
    doit3(A);
    doit4(A);
    doit5(A);
    doit6(A);
    doit7(A); // A的实例能用在所有7个函数版本中

    doit4(B);
    doit6(B);
    doit7(B); // B的实例只能用在3个函数版本中

    doit7(C); // C的实例只能用在1个函数版本中
}

// 演示了如何对带类型参数的结构体在实现方法的时候，按需求施加约束
fn demo2() {
    use std::fmt::Display;

    struct _Pair<T> {
        x: T,
        y: T,
    }

    impl<T> _Pair<T> {
        // 第一次 impl
        // new 方法不需要任何约束
        fn _new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    // Rust 中对类型是可以多次 impl 的
    impl<T: Display + PartialOrd> _Pair<T> {
        // 第二次 impl
        fn _cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
}

// 孤儿规则
// 为了不导致混乱，Rust 要求在一个模块中，如果要对一个类型实现某个 trait，
// 这个类型和这个 trait 其中必须有一个是在当前模块中定义的
// 我们想给一个外部类型实现一个外部 trait，这是不允许的。Rustc 小助手提示我们，如果实在想用的话，可以用 Newtype 模式
fn _orphan() {
    println!("\n***** orphan *****");
    use std::fmt::Display;

    struct A;
    impl Display for A {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "A")
        }
    }

    trait TraitA {}
    impl TraitA for u32 {}

    // 这样不行，因为 Display 是一个外部 trait，而 u32 也是一个外部类型
    // use std::fmt::Display;

    // impl Display for u32 {}

    // 实在想用的话，可以用 Newtype 模式
    struct MyU32(u32); // 用 MyU32 代替 u32
    impl Display for MyU32 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "MyU32")
        }
    }
    impl MyU32 {
        fn _get(&self) -> u32 {
            // 需要定义一个获取真实数据的方法 self.0 }
            self.0
        }
    }
}

// Blanket Implementation 🙆🏻统一实现
fn blanket_implementation() {
    println!("\n***** blanket_implementation *****");
    trait TraitA {
        // fn play(&self) {
        //     println!("1");
        // }
        fn play(&self);
    }
    trait TraitB {}
    // 表示对于任何实现了 TraitB 的类型 T，它们自动地也实现了 TraitA
    impl<T: TraitB> TraitA for T {
        // 如果没有默认实现，需要在这里实现
        fn play(&self) {
            println!("1");
        }
    } // 这里直接对 T 进行实现 TraitA

    impl TraitB for u32 {}
    let a = 10u32;
    a.play(); // 调用 TraitA 上实现的 play() 方法
}

fn main() {
    // trait 里面可以包含关联函数、关联类型和关联常量
    basic_trait();

    // 关联类型
    associated_type();

    // 在 T 上使用关联类型
    use_associated_type_in_t();

    // 在约束中具化关联类型
    associated_type_in_constraint();

    // 对关联类型的约束
    constraint_for_associated_type();

    // 加强对关联类型的约束
    strengthen_constraints_on_associated_types();

    // 关联常量
    associated_constant();

    // where
    where_in_trait();

    // 约束依赖
    constraint_dependence();

    // 约束中同名方法的访问
    access_to_methods_with_the_same_name_in_constraints();

    demo2();

    // 约束可按需配置
    constraint_as_needed();

    // 统一实现 Blanket Implementation
    blanket_implementation();
}
