use crate::models::Student;
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;

pub fn client() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("连接服务器失败");
    println!("已连接到服务器");

    // 启动接收线程
    let mut stream_clone = stream.try_clone().unwrap();
    thread::spawn(move || {
        let reader = BufReader::new(stream_clone);
        for line in reader.lines() {
            if let Ok(data) = line {
                if let Ok(student) = serde_json::from_str::<Student>(&data) {
                    println!("收到广播: {:?}", student);
                }
            }
        }
    });

    // 主线程处理用户输入
    loop {
        println!("\n请输入学生信息（格式：姓名,学号,年龄）或输入 'quit' 退出：");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("读取输入失败");

        if input.trim() == "quit" {
            break;
        }

        let parts: Vec<&str> = input.trim().split(',').collect();
        if parts.len() != 3 {
            println!("输入格式错误，请使用：姓名,学号,年龄");
            continue;
        }

        let student = Student {
            name: parts[0].to_string(),
            id: parts[1].to_string(),
            age: parts[2].parse().unwrap_or(0),
        };

        let json = serde_json::to_string(&student).unwrap();
        if let Err(_) = writeln!(stream, "{}", json) {
            println!("发送失败");
            break;
        }
    }
}