// async 代码的传染性
// 1. async 函数或 block 只有被 .await 后才能被驱动
// 2. .await 只能在 async 函数或 block 中使用
// Rust中有一个语法也具有传染性: 类型参数 T

async fn async_infectious() {
    println!("****** async 代码的传染性 ******");
    // 定义为一个异步函数
    async fn foo1() -> u32 {
        100
    }

    // 在 foo2 中调用 foo1, 那么这个 foo2 也必须是 async 函数
    async fn foo2() -> u32 {
        foo1().await
    }

    // foo3 同理
    async fn foo3() -> u32 {
        foo2().await
    }
    foo3().await;
}

// async 代码中的同步代码
// 1. 一类是 vec.push() 这种 API 调用, 直接使用即可
// 2. 另一类是需要执行很长时间的操作
//    2.1 可以直接使用, 但是会有性能问题, 它会阻塞当前正在跑这个异步代码的系统线程（OS Thread，由 tokio 来管理维护）
//    2.2 可以使用 task::spawn_blocking() 来解决
//    2.3 只需要将 CPU 密集型的任务放到 task::spawn_blocking 内即可
async fn sync_in_async() {
    println!("****** async 代码中的同步代码 ******");
    // 此任务跑在一个单独的线程中
    let blocking_task = tokio::task::spawn_blocking(|| {
        // 在这里可以执行阻塞线程的代码(耗时)
    });
    // 等待任务完成
    blocking_task.await.unwrap();
}
// sync 代码中的 async 代码
// 场景: 在一大堆同步代码中(std Rust), 调用一个异步函数
// 1. 使用 tokio::runtime::Builder 创建一个 runtime
// 2. 使用 runtime.block_on() 来驱动异步函数
fn _async_in_sync() {
    println!("****** sync 代码中的 async 代码 ******");
    async fn foo1() -> u32 {
        10
    }
    fn foo() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let num = rt.block_on(foo1()); // 调用了异步函数 foo1

        // 或者使用下面的写法
        // let num = rt.block_on(async { foo1().await });
        println!("num: {}", num);
    }
    foo();
}

// Rust async 实现原理
// 1. async/.await 是语法糖
// 2. 一个 async fn 会被转化成 struct FutureA 以及 impl Future for FutureA

// Rust 没有像 Go 和 Java 那样, 在系统级线程的基础上，单独实现了一层结合 GC 内存管理且具有完整屏蔽性的轻量级线程
// 在结构体之上构建一个状态机，以零成本抽象（zero cost abstract）为原则，尽量少地引入额外的消耗，配合 async/.await 语法糖，来达到简化程序员开发难度的效果

// 其他一些要注意的问题
// 1. 目前无法在 trait 中定义 async fn
//    1.1 解决这个问题可以引入 async_trait create 的 async trait 暂时过渡
//        在定义 trait 和 impl trait 的时候, 都需要添加 #[async_trait] 属性宏来标注
//        添加了这个宏标注后，trait 里的 async fn 就可以像普通的 async fn 那样在异步代码中被调用了




#[tokio::main]
async fn main() {
    async_infectious().await;

    sync_in_async().await;

    // _async_in_sync(); // 要执行这个 fn, 需要将 main 的 async 去掉, 然后上面的 tokio::main 也去掉
}
