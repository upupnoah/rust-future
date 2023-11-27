// Problem: 多个任务操作共享数据(这里数据库简化为 Vec)

// 方案 1: 全局变量
// static mut DB: Vec<u32> = Vec::new(); // 全局变量

// fn main() {
//     // 方案 1:全局变量(failed)
//     // 全局变量不能直接操作,需要用到 unsafe的功能(不推荐)
//     // DB.push(10);
// }

use std::sync::{atomic::AtomicU32, Arc};

use tokio::sync::{Mutex, RwLock};

#[tokio::main]
async fn main() {
    // 方案 2: main 函数中的对象
    // 具体操作: 在 main 中创建一个对象, 他的生命周期和 main 一样, 就相当于是全局变量了
    // let mut db: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 操作 1: 使用 move 修饰, 使得闭包中的变量不再是引用, 而是直接拿到所有权, 但是这样还是不行, 所有权转移进去了
    // let task_a = tokio::task::spawn(async move {
    //     db[4] = 50;
    // });
    // let task_b = tokio::task::spawn(async move {
    //     db[4] = 100;
    // });

    // 操作 2: 使用 Arc: Arc 可以让多个持有者共享对同一资源的所有权, 但是还是不行, Arc<T> 无法修改里面的值
    // let arc_db = Arc::new(db);
    // let arc_db2 = arc_db.clone();

    // let task_a = tokio::task::spawn(async move {
    //     arc_db[4] = 50;
    // });

    // let task_b = tokio::task::spawn(async move {
    //     arc_db[4] = 100;
    // });

    // 操作 3: Arc + Mutex
    // 在 Rust 中，Arc> 是一对很常见的组合，利用它们的组合技术，基本上可以满足绝大部分的并发编程场景
    // Arc、Mutex 和 clone() 一起，被社区叫做“Rust 三板斧”，就是因为它们简单粗暴，方便好用
    let db: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let arc_db = Arc::new(Mutex::new(db)); // 加锁
    let arc_db2 = arc_db.clone();
    let arc_db3 = arc_db.clone();

    let task_a = tokio::task::spawn(async move {
        let mut db = arc_db.lock().await; // 获得锁: lock() 返回一个 MutexGuard, 里面有一个 DerefMut, 所以可以直接操作
        db[4] = 50;
        assert_eq!(db[4], 50); // 校验值
    });
    let task_b = tokio::task::spawn(async move {
        let mut db = arc_db2.lock().await;
        db[4] = 100;
        assert_eq!(db[4], 100);
    });
    _ = task_a.await.unwrap();
    _ = task_b.await.unwrap();
    println!("{:?}", arc_db3.lock().await);

    // ******************************

    // 锁
    // tokio::sync::Mutex: 互斥锁 -> 无论是读还是写, 一次只能有一个任务访问
    // 适合场景: 读写都很多

    // tokio::sync::RwLock: 读写锁 -> 读的时候可以有多个任务访问, 写的时候只能有一个任务访问
    // 适合场景: 读多写少

    // std::sync::atomic -> 原子锁 -> 一般用于计数器
    // 如果共享数据的只是一些简单的类型，比如 bool、i32、u8、usize 等等，就不需要使用 Mutex 或 RwLock 把这些类型包起来，
    // 比如像这样 Arc>，可以直接用 Rust 标准库里提供的原子类型。std::sync::atomic 这个模块下面提供了很多原子类型，
    // 比如 AtomicBool、AtomicI8、AtomicI16 等等

    // Example: tokio::sync::RwLock
    let lock = RwLock::new(5);
    // 多个读锁可以同时存在
    {
        let r1 = lock.read().await;
        let r2 = lock.read().await;
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    } // 这里 r1 和 r2 的生命周期结束, 两个读锁释放

    // 同时只能存在一个写锁
    {
        let mut w = lock.write().await;
        *w += 1;
        assert_eq!(*w, 6);
    } // 在这一句结束时, 写锁释放掉

    // Example: std::sync::atomic
    // 创建
    let atomic_forty_two = AtomicU32::new(42);
    let _arc_data = Arc::new(atomic_forty_two);

    let mut some_var = AtomicU32::new(10);

    // 更新
    *some_var.get_mut() = 5;
    assert_eq!(*some_var.get_mut(), 5);
}
