use crate::models::Student;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

pub fn client() {
    // 连接服务器
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("连接服务器失败");
    println!("已连接到服务器");

    // 启动接收线程
    let mut stream_clone = stream.try_clone().unwrap();
    thread::spawn(move || {
        let mut buffer = [0; 1024];
        let mut message = String::new();

        println!("开始监听广播消息...");
        loop {
            match stream_clone.read(&mut buffer) {
                Ok(0) => {
                    println!("服务器关闭连接");
                    break;
                }
                Ok(n) => {
                    // 将接收到的字节转换为字符串
                    if let Ok(str_data) = String::from_utf8(buffer[..n].to_vec()) {
                        message.push_str(&str_data);

                        // 检查是否收到完整的 JSON 消息
                        if let Ok(student) = serde_json::from_str::<Student>(&message) {
                            println!("收到其他客户端的广播消息: {:?}", student);
                            message.clear(); // 清空消息缓冲区
                        }
                    }
                }
                Err(e) => {
                    println!("读取错误: {}", e);
                    break;
                }
            }
        }
    });

    // 模拟数据
    let students = vec![
        Student {
            name: "张三".to_string(),
            id: "2024001".to_string(),
            age: 20,
        },
        Student {
            name: "李四".to_string(),
            id: "2024002".to_string(),
            age: 21,
        },
        Student {
            name: "王五".to_string(),
            id: "2024003".to_string(),
            age: 22,
        },
    ];

    // 发送模拟数据
    for student in students {
        let json = serde_json::to_string(&student).unwrap() + "\n";
        println!("准备发送学生信息: {:?}", student);
        if let Err(e) = stream.write_all(json.as_bytes()) {
            println!("发送错误: {}", e);
            break;
        }
        println!("成功发送学生信息");
        thread::sleep(Duration::from_secs(1)); // 每秒发送一条数据
    }

    // 保持连接一段时间以接收广播
    println!("等待接收其他客户端的广播消息...");
    thread::sleep(Duration::from_secs(5));
}
