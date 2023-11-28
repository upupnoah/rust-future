use std::time::Duration;

use tokio::sync::{mpsc, oneshot};
use tokio::task::spawn;

// 使用 channel 在并发中避免使用锁

async fn tokio_channel() {
    println!("****** tokio_channel ******\n");
    // 使用 tokio 的 MPSC channel
    // MPSC: 多生产者, 单消费者
    // 多个 tx, 1 个 rx
    let mut db: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // let (tx, mut rx) = mpsc::channel::<u32>(100);

    // let tx1 = tx.clone();
    // let tx2 = tx.clone();

    // let task_a = spawn(async move {
    //     println!("in task_a 1");
    //     tokio::time::sleep(Duration::from_secs(3)).await; // 等待 3 秒
    //     println!("in task_a 2");

    //     // 发送端标准写法
    //     if let Err(_) = tx1.send(50).await {
    //         println!("receiver dropped");
    //         return;
    //     }
    // });
    // let task_b = spawn(async move {
    //     println!("in task_b");
    //     // 发送端标准写法
    //     if let Err(_) = tx2.send(100).await {
    //         println!("receiver dropped");
    //         return;
    //     }
    // });
    // let task_c = spawn(async move {
    //     // 接收端标准写法

    //     // 当收到一个 None 时，意味着发送端已经被关闭了，所以我们可以退出循环
    //     while let Some(i) = rx.recv().await {
    //         println!("got = {}", i);
    //         db[4] = i;
    //         println!("{:?}", db);
    //     }
    // });

    // tokio::task::spawn() 这个 API 有个特点
    // 就是通过它创建的异步任务，一旦创建好，就会立即扔到 tokio runtime 里执行，
    // 不需要对其返回的 JoinHandler 进行 await 才驱动执行，这个特性很重要

    // 还有一个 无背压的 channel: tokio::sync::unbounded_channel
    // 无背压的 channel 会一直缓存消息，有内存耗尽的风险

    // 增加一个需求：我在 task_c 中将 db 更新完成，想给 task_a 和 task_b 返回一个事件通知说，我已经完成了，应该怎么做？
    // tokio::sync::oneshot: 一次性 channel
    // 适合场景: 一次性传递消息(例如计算结果返回场景)
    // 这里我们 MPSC + Oneshot 两种通道成功实现了 Request/Response 模式
    let (tx, mut rx) = mpsc::channel::<(u32, oneshot::Sender<bool>)>(100);

    let tx1 = tx.clone();
    let tx2 = tx.clone();
    let task_a = spawn(async move {
        tokio::time::sleep(Duration::from_secs(3)).await; // 等待 3 秒
        let (resp_tx, resp_rx) = oneshot::channel();
        if let Err(_) = tx1.send((50, resp_tx)).await {
            println!("receiver dropped");
            return;
        }
        if let Ok(ret) = resp_rx.await {
            if ret {
                println!("task_a finished with success.");
            } else {
                println!("task_a finished with failure.");
            }
        } else {
            println!("oneshot sender dropped.");
            return;
        }
    });

    let task_b = spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        if let Err(_) = tx2.send((100, resp_tx)).await {
            println!("receiver dropped");
        }
        if let Ok(ret) = resp_rx.await {
            if ret {
                println!("task_a finished with success.");
            } else {
                println!("task_a finished with failure.");
            }
        } else {
            println!("oneshot sender dropped.");
            return;
        }
    });

    let task_c = spawn(async move {
        // 接收端标准写法

        // 当收到一个 None 时，意味着发送端已经被关闭了，所以我们可以退出循环
        while let Some((v, resp_tx)) = rx.recv().await {
            println!("got = {}", v);
            db[4] = v;
            println!("{:?}", db);
            resp_tx.send(true).unwrap(); // unwrap 处理了 Result, 如果发送失败, 会 panic
        }
    });
    // 代理: 拿到 db 的所有权
    // 对于 main 函数来说, 会一直阻塞在这里, 直到 task_c 执行完毕
    // 但是 使用 tokio::task::spawn() 创建的异步任务, 本身就是并发执行的关系, 所以虽然没有执行下面的两个 task a/b, 但是不影响结果
    _ = task_c.await.unwrap();

    _ = task_a.await.unwrap();
    _ = task_b.await.unwrap();

    // 其他 channel
    // broadcast channel: 广播模式, 实现了 MPMC 模型,多生产者多消费者, 可以用来实现发布 - 订阅模式
    //     每个消费者都会收到每个生产者发出的同样的消息副本
    //     broadcast 实际已经覆盖 SPMC 模型, 所以不用再单独定义 SPMC 了
    // watch channel: 监听模式, 用来实现观察者模式
    //     1. 只有一个生产者, 多个消费者
    //     2. 只关心最后一个值
    //     它适用于一些特定的场景，比如配置更新需要通知工作任务重新加载，平滑关闭任务等等
}

// 任务管理的 2 种 常见模式!!!!
// 1. 等待所有任务一起返回
// 这个模式有个特点，就是要等待前面任务结束，才能拿到后面任务的返回结果。
// 如果前面某个任务执行的时间比较长，即使后面的任务实际已经执行完了，在最后搜集结果的时候，
// 还是需要等前面那个任务结束了后，我们才能搜集到后面任务的结果
async fn wait_for_all_tasks_return_together1() {
    println!("****** wait_for_all_tasks_return_together1 ******\n");
    async fn my_background_op(id: i32) -> String {
        let s = format!("Starting background task {}.", id);
        println!("{}", s);
        s
    }
    let ops = vec![1, 2, 3];
    let mut tasks = Vec::with_capacity(ops.len());
    for op in ops {
        // 任务创建后, 立即开始运行, 我们用一个 Vec 来持有各个任务的 handler
        tasks.push(spawn(my_background_op(op)));
    }
    let mut outputs = Vec::with_capacity(tasks.len());
    for task in tasks {
        // 任务执行完毕后, 我们用一个 Vec 来持有各个任务的输出
        outputs.push(task.await.unwrap());
    }
    println!("outputs: {:?}", outputs);
}

async fn wait_for_all_tasks_return_together2() {
    println!("\n****** wait_for_all_tasks_return_together2 ******\n");
    // 总共的等待时间差不多是 3 秒, 而不是 6 秒
    // 因为在 a 执行的时候, b/c 已经开始执行了, 所以总共的等待时间是 3 秒
    let task_a = spawn(async move {
        println!("in task_a");
        tokio::time::sleep(Duration::from_secs(3)).await; // 等待 3 秒
        println!("sleep over");
        1
    });
    let task_b = spawn(async move {
        println!("in task_b");
        tokio::time::sleep(Duration::from_secs(2)).await; // 等待 2 秒
        2
    });
    let task_c = spawn(async move {
        println!("in task_c");
        tokio::time::sleep(Duration::from_secs(1)).await; // 等待 1 秒
        3
    });
    let mut tasks = Vec::with_capacity(3);

    tasks.push(task_a);
    tasks.push(task_b);
    tasks.push(task_c);

    let mut outputs = Vec::with_capacity(tasks.len());
    for task in tasks {
        outputs.push(task.await.unwrap());
    }
    println!("outputs: {:?}", outputs);
}

// 2. 等待任意一个任务返回 (使用 tokio::select!)
// 在实际场景中，还有另外一大类需求，就是在一批任务中，哪个任务先执行完，就马上返回那个任务的结果。
// 剩下的任务，要么是不关心它们的执行结果，要么是直接取消它们继续执行
async fn wait_for_one_task_first() {
    println!("\n****** wait_for_one_task_first ******\n");
    let task_a = spawn(async move {
        println!("in task_a");
        tokio::time::sleep(Duration::from_secs(3)).await; // 等待 3 秒
        println!("sleep over");
        1
    });
    let task_b = spawn(async move {
        println!("in task_b");
        2
    });
    let task_c = spawn(async move {
        println!("in task_c");
        3
    });

    let ret = tokio::select! {
        r = task_a => r.unwrap(),
        r = task_b => r.unwrap(),
        r = task_c => r.unwrap(),
    };

    // ret 不是 2,就是 3, 因为 task_a 会阻塞 3 秒, 所以 task_b/c 会先执行完毕
    println!("ret: {}", ret);
}
#[tokio::main]
async fn main() {
    // 按照学习的顺序应当是 ①, ②, ③, ④, 但是由于 ① 会阻塞, 所以先执行 ②, ③, ④
    wait_for_all_tasks_return_together1().await; // ②
    wait_for_all_tasks_return_together2().await; // ③

    // ② 和 ③ 都是在所有任务中等待最长的那个任务执行完成后，统一返回

    wait_for_one_task_first().await; // ④

    tokio_channel().await; // ①
}
