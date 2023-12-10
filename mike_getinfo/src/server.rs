use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use std::env;
use tokio::net::TcpListener;
use tokio::process::Command;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

// 基于 tokio 实现 tcp server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = env::args().nth(1).unwrap_or("127.0.0.1:8888".to_string());
    println!("Listening on: {}", addr);

    // ?的作用: Ok 则解开, Err 则返回
    let listener = TcpListener::bind(&addr).await?;

    // 无限循环, 表明始终处于服务状态
    // 这个 loop 的作用: 一直等待客户端的连接, 一旦有客户端连上来, 就创建一个新的任务去处理
    loop {
        // 等待客户端请求连上来
        // let (mut socket, _) = listener.accept().await?;
        // 上面这一行代码相当于下面
        // let tmp = listener.accept().await;
        // let (mut socket, _) = match listener.accept().await {
        //     Ok(result) => result,
        //     Err(e) => return Err(e.into()),
        // };

        // 来一个客户端连接, 创建一个对应的新任务
        // tokio::spawn(async move {
        //     // 为每个连接分配一个缓冲区
        //     let mut buf = [0; 1024];
        //     let mut offset = 0;

        //     // 循环读, 因为不能确保一次能从网络上读完数据
        //     loop {
        //         // 读操作,返回的 n 表示读了多少个字节
        //         // 正常情况下,读到数据才会返回, 如果没有读到, 就会等待
        //         let n = socket
        //             .read(&mut buf[offset..])
        //             .await
        //             .expect("failed to read data from socket");

        //         // 如果读到 0 字节, 表示碰到了 EOF, 表明远端的写操作已断开,这个一定要判断
        //         if n == 0 {
        //             // 碰到了 EOF 就直接结束任务, 因为后面的操作没有意义
        //             return;
        //         }
        //         println!("offset: {offset}, n : {n}");
        //         let end = offset + n;
        //         // 转换指令为字符串
        //         if let Ok(directive) = std::str::from_utf8(&buf[..end]) {
        //             println!("{directive}");
        //             // 执行指令对应的工作
        //             let output = process(directive).await;
        //             println!("{output}");

        //             // 向客户端返回处理结果
        //             socket
        //                 .write_all(&output.as_bytes())
        //                 .await
        //                 .expect("failed to write data to socket");
        //         } else {
        //             // 判断是否转换失败,如果失败,就有可能是网络上的数据还没读完
        //             // 要继续 loop 读下一波数据
        //             offset = end;
        //         }
        //     }
        // });

        // 使用 Frame 来处理上面的代码
        let (stream, _) = listener.accept().await?;
        // 包裹成一个 Frame stream
        let mut framed_stream = Framed::new(stream, LengthDelimitedCodec::new());

        // 创建子 task 执行任务
        tokio::spawn(async move {
            // 等待读取一个一个 msg, 如果返回 None, 会退出这个循环
            while let Some(msg) = framed_stream.next().await {
                match msg {
                    Ok(msg) => {
                        // 解析指令, 执行任务
                        let directive = String::from_utf8(msg.to_vec())
                            .expect("error when converting to string directive.");
                        let output = process(&directive).await;
                        println!("{output}");

                        // 返回执行结果
                        _ = framed_stream.send(Bytes::from(output)).await;
                    }
                    Err(e) => {
                        println!("{e:?}");
                    }
                }
            }
        });
    }
}

async fn process(directive: &str) -> String {
    if directive == "gettime" {
        // 这里我们用了 unwrap() 是因为我们一般确信执行 date 命令不会失败
        // let output = Command::new("date").output().await.unwrap();
        // String::from_utf8(output.stdout).unwrap()

        // 更可靠的做法是对返回的 Result 进行处理
        match Command::new("date").output().await {
            Ok(output) => match String::from_utf8(output.stdout) {
                Ok(s) => s,
                Err(_) => "Error converting output to string".to_string(),
            },
            Err(_) => "Failed to execute command".to_string(),
        }
    } else {
        // 如果是其他指令, 我们目前返回无效指令
        "invalid command".to_owned()
    }
}
