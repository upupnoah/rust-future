fn box_t_demo() -> Box<u32> {
    println!("***** box_t_demo *****");
    let i = 100u32;
    Box::new(i)
}
#[derive(Debug)]
struct Point {
    _x: i32,
    _y: i32,
}

fn box_t_struct() -> Box<Point> {
    println!("\n***** box_t_struct *****");
    let p = Point { _x: 10, _y: 10 };

    // 把 p 实例强行按位复制了一份，并且放到了堆上，我们记为 p'。
    // 然后 box_t_struct() 函数返回，把 Box 指针实例 move 给了 _p。之后，_p 拥有了对 p' 的所有权

    // 需要区分是 copy 语义 还是 move 语义
    // 结构体 默认是 move 语义
    // 整数类型 默认是 copy 语义

    // Box 在 new 的时候, 会根据语义的不同, 会有不同的结果
    // Box 会获得所有权, 并且会在堆上分配内存
    // Box 本身的语义是 move 语义
    Box::new(p)
}

fn box_deref() {
    println!("\n***** box_deref *****");
    let val = 5;
    let boxed = Box::new(val);

    // 使用 * 解引用
    // * 是 Box::new() 的逆操作
    assert_eq!(5, *boxed);

    let boxed = Box::new(5);
    let val: u8 = *boxed;
    println!("{:?}", val);
    println!("{:?}", boxed); // 用于u8类型，解引用后，boxed实例还能用, 因为 u8 是 copy 语义

    let sboxed = Box::new(Point { _x: 10, _y: 10 });
    let sval: Point = *sboxed;
    println!("{:?}", sval);
    // println!("{:?}", sboxed); // 用于结构体类型，解引用后，sboxed实例不能用

    // 一种类型，被 Box<> 包起来的过程就叫作这个类型的盒化 -> boxed
}

fn use_box_like_t() {
    println!("\n***** use_box_like_t *****");
    // 标准库为 Box<T> 实现了 Deref, Drop, AsRef<T> 等 Trait, 因此 Box<T> 可以像 T 一样使用
    impl Point {
        fn play(&self) {
            println!("I'm a method of Point.");
        }
    }
    let boxed = Box::new(Point { _x: 10, _y: 20 });
    boxed.play();
    println!("{:} {:}", boxed._x, boxed._y);

    // Box<T> 拥有对 T 实例的所有权, 所以可以对 T 实例进行写操作
    let mut boxed = Box::new(Point { _x: 10, _y: 20 });
    *boxed = Point { _x: 20, _y: 30 }; // 这一行，使用解引用操作更新值
    println!("{:?}", boxed);
}

fn clone_on_box_t() {
    println!("\n***** clone_on_box_t *****");
    // Box<T> 的 Clone
    // Box<T> 能否 Clone,需要看 T 是否实现了 Clone, 因为也需要把 T 的资源 Clone 一份
    #[derive(Debug, Clone)]
    struct Point2 {
        _x: u32,
        _y: u32,
    }
    impl Point2 {
        fn _play(&self) {
            println!("I'am a method of Point.");
        }
    }
    let boxed = Box::new(Point2 { _x: 10, _y: 20 });
    let mut another_boxed = boxed.clone();
    *another_boxed = Point2 { _x: 200, _y: 300 };
    println!("{:?}", boxed);
    println!("{:?}", another_boxed);
}

fn box_t_as_fn_param() {
    println!("\n***** box_t_as_fn_param *****");
    #[derive(Debug)]
    struct Point3 {
        _x: u32,
        _y: u32,
    }

    fn foo(p: Box<Point3>) {
        // 这里参数类型是 Box<Point>
        println!("{:?}", p);
    }

    foo(Box::new(Point3 { _x: 10, _y: 20 }));
}

fn ref_box() {
    println!("\n***** ref_box *****");

    let mut boxed = Box::new(Point { _x: 10, _y: 20 });
    boxed.play();
    let y = &boxed;
    y.play();
    println!("{:?}", y);

    let y = &mut boxed;
    y.play();
    println!("{:?}", y);
    **y = Point { _x: 999, _y: 999 }; // 注意这里用了二级解引用
    println!("{:?}", y);
}

fn box_self() {
    println!("\n***** box_self *****");
    impl Point {
        fn paly_ref(&self) {
            println!("I'm a play_ref method of Point.");
        }
        fn play_mutref(&mut self) {
            println!("I'm a play_mutref method of Point.");
        }
        fn play_own(self) {
            println!("I'm a play_own method of Point.");
        }
        fn _paly_boxown(self: Box<Self>) {
            println!("I'm a paly_boxown method of Point.");
        }
    }
    let mut boxed = Box::new(Point { _x: 10, _y: 20 });
    boxed.paly_ref();
    boxed.play_mutref();
    boxed.play_own();
    // boxed._paly_boxown();
}

fn box_in_struct() {
    println!("\n***** box_in_struct *****");
    // Box<T> 作为一个类型, 自然是可以作为 结构体的 Field 的
    struct Triangle {
        _one: Box<Point>, // 三个字段都是 Box<Point> 类型
        _two: Box<Point>,
        _three: Box<Point>,
    }
    let _t = Triangle {
        _one: Box::new(Point { _x: 10, _y: 20 }),
        _two: Box::new(Point { _x: 10, _y: 20 }),
        _three: Box::new(Point { _x: 10, _y: 20 }),
    };
}

fn box_dyn_trait() {
    println!("\n***** box_dyn_trait *****");
    // 之前学过, dyn trait 是一种类型
    // 但是 dyn trait 本身的大小是不确定的, 所以 dyn trait 的出现总是要借助于 引用或 智能指针
    // Box<dyn trait> 是最常见的, 甚至比 &dyn trait 更常见
    // 原因: Box<dyn trait> 有所有权, 而 &dyn trait 没有所有权(没有所有权有时候不方便)

    #[derive(Debug, Clone, Copy)]
    struct Atype;
    struct Btype;
    struct Ctype;

    trait TraitX {}

    impl TraitX for Atype {}
    impl TraitX for Btype {}
    impl TraitX for Ctype {}

    fn doit(_x: Box<dyn TraitX>) {}
    let a = Atype;

    // 这里的 a 是 Atype 类型, 但是被转换成了 Box<dyn TraitX> 类型
    // 可以这样转换是因为 Atype 实现了 TraitX
    doit(Box::new(a));
    let b = Btype;
    doit(Box::new(b));
    let c = Ctype;
    doit(Box::new(c));

    // 如果 dyn trait 出现在结构体里，那么 Box<dyn trait> 形式就比 &dyn trait 形式要方便得多
    struct MyStruct {
        _x: Box<dyn TraitX>,
    }
    let _t1 = MyStruct { _x: Box::new(a) };

    // 这里无法通过编译, 涉及到 引用的生命期 的概念, 以后再说
    // struct MyStruct1 {
    //     x: &dyn TraitX, // 结构体字段类型是 &dyn TraitX
    // }
}

fn arc_t() {
    println!("\n***** arc_t *****");
    // Box<T> 是单所有权或独占多有权模型的智能指针, 而 Arc<T> 是共享所有权模型的智能指针
    // 也就是多个变量可以同时拥有一个资源的所有权
    // Arc<T> 的全称是 Atomic Reference Counted, 也就是原子引用计数
    // 和 Box<T> 一样, Arc<T> 会保证被包装的内容被分配在堆上

    // Arc 的主要功能是和 clone() 配合使用。在 Arc 实例上每一次新的 clone() 操作，总是会将资源的引用数 +1，
    // 而保持原来那一份资源不动，这个信息记录在 Arc 实例里面。每一个指向同一个资源的 Arc 实例走出作用域，就会给这个引用计数 -1。
    // 直到最后一个 Arc 实例消失，目标资源才会被销毁释放
    use std::sync::Arc;
    let arced = Arc::new(Point { _x: 10, _y: 20 });

    // Arc 的 clone 不要求被包装的类型实现 Clone trait
    // Arc 的 clone 只是简单地将引用计数 +1
    // 所以性能是非常高的
    let another_arced = arced.clone();
    println!("{:?}", arced);
    println!("{:?}", another_arced);
    let arc3_ref = &another_arced;

    // 类似于 Box<T>, Arc<T> 也可以像 T 一样使用(因为实现了 Deref, Drop, Clone 等 Trait)
    // Arc<T> 的不可变引用也可以调用T 的方法
    arc3_ref.play();
}

fn arc_self() {
    println!("\n***** arc_self *****");
    use std::sync::Arc;

    #[derive(Debug)]
    struct PointArc {
        _x: u32,
        _y: u32,
    }

    impl PointArc {
        fn play_ref(&self) {
            println!("I'am play_ref of Point.");
        }
        fn play_mutref(&mut self) {
            println!("I'am play_mutref of Point.");
        }
        fn _play_own(self) {
            println!("I'am play_own of Point.");
        }
        fn play_boxown(self: Box<Self>) {
            // 注意这里
            println!("I'am play_boxown of Point.");
        }
        fn play_arcown(self: Arc<Self>) {
            // 注意这里
            println!("I'am play_arcown of Point.");
        }
    }

    let mut boxed: Box<PointArc> = Box::new(PointArc { _x: 10, _y: 20 });
    boxed.play_ref();
    boxed.play_mutref();
    boxed.play_boxown();
    // boxed.play_own();  // play_boxown()和 play_own() 只能同时打开一个

    let arced: Arc<PointArc> = Arc::new(PointArc { _x: 10, _y: 20 });
    arced.play_ref();
    // arced.play_mutref();  // 不能用
    // arced._play_own();     // 不能用，Arc<T> 中的T无法被移出
    arced.play_arcown();

    // 不能通过 Arc<> 直接修改里面类型的值，也不能像 Box<> 的解引用操作那样，把里面的内容从 Arc<> 中移动出来

    // 之前的 Box<dyn trait> 可以改成 Arc<dyn trait>

    // 值的修改
    // Arc<T> 不能修改,虽然 Arc<T> 拥有所有权, 但是 Arc<T> 不提供修改 T 的能力
    // 后面会讲到 Mutex、RwLock 等锁
    // 想要修改 Arc 里面的内容，必须配合这些锁才能完成，比如 Arc<Mutex<T>> 或 Arc<RwLock<T>>

    // 其实很好理解，共享所有权的场景下，如果任意一方能随意修改被包裹的值，那就会影响其他所有权的持有者，整个就乱套了。
    // 所以要修改的话必须引入锁的机制

    // Arc<T> 与不可变引用 & 的区别
    // 他们都是共享对象的行为,本质上都是指针。但 Arc<T> 是共享了所有权模型, 而 & 只是共享借用模型
    // 共享借用模型就得遵循借用检查器的规则——借用的有效性依赖于被借用资源的 scope。对于这个的分析是非常复杂的。
    // 而所有权模型是由自己来管理资源的 scope，所以处理起来比较方便

    // 其实 Rust 里还有很多智能指针，比如 Rc、Cell、RefCell 等等，每一种智能指针类型都有自己的特点
}
fn main() {
    // 堆上的资源，默认与整个程序进程的存在时间一样久
    let _i = box_t_demo();
    let _p = box_t_struct();

    box_deref();

    use_box_like_t();

    clone_on_box_t();

    box_t_as_fn_param();

    ref_box();

    box_self();

    box_in_struct();

    box_dyn_trait();

    arc_t();

    arc_self();
}
