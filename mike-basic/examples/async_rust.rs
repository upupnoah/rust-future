// fn foo() {
//     let a = async {};
//     a.await;
// }

// fn foo() {
//     async {}.await;
// }

// 上方两个代码无法通过编译, 因为 await 关键字只能在 async 块 或 函数里使用

// async fn foo() {
//     let a = async {};
//     a.await;
// }

// *****************************************************

// main 函数前不能加 async 修饰
// 可以使用 tokio 库提供的属性宏标注, 然后就可以在 main 函数里使用 await 了
// #[tokio::main]
// async fn main() {
//     let a = async {};
//     a.await;
// }

// 上面的代码相当于下方的代码
// fn main() {
//     // 链式调用风格，这个风格在其他语言中也很普遍，只要遵循前一个函数调用返回自身或者新的对象即可
//     tokio::runtime::Builder::new_multi_thread()
//         .enable_all()
//         .build()
//         .unwrap()
//         .block_on(async {
//             // 注意这里block_on，里面是异步代码
//             println!("Hello world");
//         })
// }

// tokio 还可以基于当前系统线程创建 单线程 的 Runtime(运行时)
// #[tokio::main(flavor = "current_thread")] // 属性标注里面的配置参数
// async fn main() {
//     // 展开后的代码相当于上面注释代码中的第一行改为: tokio::runtime::Builder::new_current_thread()
//     println!("Hello world");
// }

use tokio::fs::File;
use tokio::io::AsyncWriteExt; // 引入 AsyncWriteExt trait

// 文件写
async fn write_to_file() -> std::io::Result<()> {
    println!("***** 文件写 *****");
    let mut file = File::create("./examples/foo.txt").await.unwrap(); // 创建文件
    file.write_all(b"hello, world!").await.unwrap(); // 写入内容
    Ok(())
}

// 文件读
use tokio::io::AsyncReadExt;
async fn read_file() -> std::io::Result<()> {
    println!("\n***** 文件读 *****");
    let mut file = File::open("./examples/foo.txt").await.unwrap(); // 打开文件
    let mut contents = vec![];
    // 将文件内容读到 contents 动态数组内, 注意传入的是可变引用
    file.read_to_end(&mut contents).await.unwrap();
    // println!("len = {}", contents.len());
    Ok(())
}

use std::time::Duration;
use tokio::time;
// 定时器
async fn timer() {
    println!("\n***** 定时器 *****");
    let mut interval = time::interval(Duration::from_secs(3)); // 每隔 3 秒触发一次

    for _ in 0..2 {
        interval.tick().await; // 等待下一次触发
        println!("tick");
    }

    // 滴答，立即执行
    // interval.tick().await;
    // // 滴答，这个滴答完成后，3s过去了
    // interval.tick().await;
    // // 滴答，这个滴答完成后，3s过去了
    // interval.tick().await;
}

// tokio 组件
// Runtime 设施基础: 可以自由配置基于 单/多线程 的 Runtime
// 轻量级任务 task: 可以理解成 Go 语言找那个的 Goroutine 这种轻量级线程, 而不是操作系统层面的线程
// 异步输入输出(I/O): 网络模块 net、文件操作模块 fs、signal 模块、process 模块等
// 时间模块: 定时器 Interval 等
// 异步场景下的同步原语: channel、Mutex 锁等等
// 在异步环境下执行计算密集型任务的方案: spawn_blocking 等等

// tokio 的上层建筑
// Hyper: HTTP 协议 Server 和 client 的实现
// Axum: Web 开发框架
// async-graphql: GraphQL 开发框架
// tonic: gRPC 框架的 Rust 实现
// ...

async fn task_handler() {
    println!("\n***** task_handler *****");
    // 在这里执行异步任务
    let task_a = task::spawn(async { "hello world!" });
    // ...

    // 等待子任务结束，返回结果
    //task_a.await 会返回一个 Result，所以上面代码中，需要加一个 unwrap() 把 task_a 真正的返回内容解包出来。
    //至于对 task 的 .await 为什么会返回一个 Result，而不是直接返回异步任务的返回值本身，是因为 task 里有可能会发生 panic
    let result = task_a.await.unwrap();
    assert_eq!(result, "hello world!");
}

async fn handler_error() {
    println!("\n***** handler_error *****");
    // let task_a = task::spawn(async { panic!("something bad happened!") });
    // 当task_a里面panic时，对task handler进行.await，会得到Err
    // assert!(task_a.await.is_err());
}

async fn more_tasks() {
    async fn my_background_op(id: i32) -> String {
        let s = format!("Starting background task {}.", id);
        println!("{}", s);
        s
    }
    let ops = vec![1, 2, 3];
    let mut tasks = Vec::with_capacity(ops.len());
    for op in ops {
        // 任务创建后, 立即开始执行, 我们用一个 Vec 来持有各个任务的 handler
        tasks.push(tokio::spawn(my_background_op(op)));
    }
    let mut outputs = Vec::with_capacity(tasks.len());

    // 我们用 tasks 这个动态数组持有 3 个异步任务的 handler，它们是并发执行的
    for task in tasks {
        outputs.push(task.await.unwrap());
    }
    println!("{:?}", outputs);
}
// 哪些操作要加 .await?
// 设计到I/O操作的,都可以加, 因为 tokio 已经帮我们实现了异步 I/O
// 最常见的 I/O 操作是网络I/O、磁盘I/O 等
// net 模块: 网络操作
// fs 模块: 文件操作
// 定时器操作: interval、sleep 等函数
// channel: 四种管道 oneshot、mpsc、watch、broadcast
// signal 模块: 系统信号处理
// process 模块: 调用系统命令等

// 在查看 tokio API 的时候, 只要接口前面有 async 关键字修饰, 那么使用的时候就需要加 .await
use tokio::task;
#[tokio::main]
async fn main() {
    let result = write_to_file().await; // 通过 .await 等待异步函数执行完毕
    println!("{:?}", result);

    let result = read_file().await;
    println!("{:?}", result);

    timer().await;

    // 创建一个 tokio task, 相对于 main task 来说,这是一个子 task
    // 在 tokio 管理下的 async fn main 本身就是一个 task
    // 在用 spawn() 创建 task_a 后，这个新任务就立即执行
    // 也可以用一个变量来接收 spawn() 的返回值，这个返回值是 JoinHandler 类型的
    task::spawn(async { // 在这里执行异步任务
    });

    // 在 tokio 中,子 task 的生存期有可能超过父 task 的生存期, 也就是父 task 执行结束了, 但子 task 还在执行
    // 如果在父 task 里要等待子 task 执行完成, 再结束自己, 保险的做法是用 JoinHandler
    // 在 main 函数中有更多细节，如果 main 函数所在的 task 先结束了，会导致整个程序进程退出，有可能会强制杀掉那些新创建的子 task
    task_handler().await;

    // 处理 panic
    // 在 Rust 中，只要过程中有可能返回错误，那就果断用 Result 包一层作为返回值，这是典型做法
    handler_error().await;

    more_tasks().await;
}