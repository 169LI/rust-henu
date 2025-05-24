use std::io::{Read, Write};
use std::net::TcpListener;
use crate::models::Student;

pub fn run_server() {
    //监听端口，等待连接
    let listener = TcpListener::bind("127.0.0.1:8080").expect("无法绑定端口");
    println!("服务器正在监听 127.0.0.1:8080");

    //接受连接
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("新的连接已建立");
                
                //读取数据
                let mut buffer = [0; 1024];
                match stream.read(&mut buffer) {
                    Ok(size) => {
                        let json_str = String::from_utf8_lossy(&buffer[..size]);
                        //反序列化
                        match serde_json::from_str::<Student>(&json_str) {
                            Ok(student) => {
                                println!("收到学生信息: {:?}", student);
                                //写入响应
                                stream.write_all("数据已接收".as_bytes()).expect("写入响应失败");
                            }
                            Err(e) => println!("反序列化错误: {}", e),
                        }
                    }
                    Err(e) => println!("读取数据错误: {}", e),
                }
            }
            Err(e) => println!("连接错误: {}", e),
        }
    }
} 