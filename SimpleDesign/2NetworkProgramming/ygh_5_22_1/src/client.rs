use std::io::{Read, Write};
use std::net::TcpStream;
use crate::models::Student;

pub fn run_client() {
    //连接服务器
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("无法连接到服务器");
    println!("已连接到服务器");

    let student = Student {
        name: String::from("张三"),
        id: String::from("S12345"),
    };
    //序列化
    let json_data = serde_json::to_string(&student).expect("序列化失败");
    stream.write_all(json_data.as_bytes()).expect("发送数据失败");
    println!("已发送学生信息: {:?}", student);

    //读取响应
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(size) => {
            let response = String::from_utf8_lossy(&buffer[..size]);
            println!("服务器响应: {}", response);
        }
        Err(e) => println!("读取响应错误: {}", e),
    }
} 