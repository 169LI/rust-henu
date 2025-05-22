use crate::models::Student;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

pub fn server() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("绑定端口失败");
    println!("服务器启动成功，等待客户端连接...");

    let clients: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let clients = Arc::clone(&clients);
                thread::spawn(move || {
                    handle_client(stream, clients);
                });
            }
            Err(err) => println!("接受连接失败：{}", err),
        }
    }
}

fn handle_client(stream: TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>) {
    let mut stream = stream;
    let reader = BufReader::new(stream.try_clone().unwrap());

    // 添加新客户端
    clients.lock().unwrap().push(stream.try_clone().unwrap());
    println!("新客户端已连接");

    for line in reader.lines() {
        match line {
            Ok(data) => {
                match serde_json::from_str::<Student>(&data) {
                    Ok(student) => {
                        println!("收到学生信息: {:?}", student);
                        broadcast(&clients, &student);
                    }
                    Err(e) => println!("解析数据失败: {}", e),
                }
            }
            Err(_) => break,
        }
    }

    // 移除断开的客户端
    let mut clients = clients.lock().unwrap();
    clients.retain(|s| s.try_clone().is_ok());
    println!("客户端已断开连接");
}

fn broadcast(clients: &Arc<Mutex<Vec<TcpStream>>>, student: &Student) {
    let json = serde_json::to_string(student).unwrap();
    let mut clients = clients.lock().unwrap();
    
    clients.retain(|stream| {
        match writeln!(stream, "{}", json) {
            Ok(_) => true,
            Err(_) => false,
        }
    });
}
