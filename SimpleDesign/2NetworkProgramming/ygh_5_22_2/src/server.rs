use crate::models::Student;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

pub fn server() {
    // 创建 TCP 监听器
    let listener = TcpListener::bind("127.0.0.1:8080").expect("服务器启动失败");
    println!("服务器启动成功，监听端口 8080...");

    // 创建客户端连接列表
    let clients: Arc<Mutex<HashMap<usize, TcpStream>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut client_id = 0;

    // 接受新的客户端连接
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let clients = Arc::clone(&clients);
                let id = client_id;
                client_id += 1;

                println!("新客户端 [ID: {}] 正在连接...", id);
                // 为每个客户端创建新线程
                thread::spawn(move || {
                    handle_client(stream, id, clients);
                });
            }
            Err(e) => println!("连接错误: {}", e),
        }
    }
}

fn handle_client(mut stream: TcpStream, id: usize, clients: Arc<Mutex<HashMap<usize, TcpStream>>>) {

    // 将新客户端添加到列表
    clients.lock().unwrap().insert(id, stream.try_clone().unwrap());
    println!("当前连接的客户端数量: {}", clients.lock().unwrap().len());

    let mut buffer = [0; 1024];
    let mut message = String::new();

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("客户端 [ID: {}] 断开连接", id);
                break;
            }
            Ok(n) => {
                // 将接收到的字节转换为字符串
                if let Ok(str_data) = String::from_utf8(buffer[..n].to_vec()) {
                    message.push_str(&str_data);
                    
                    // 尝试解析完整的 JSON 消息
                    if let Ok(student) = serde_json::from_str::<Student>(&message) {
                        println!("收到客户端 [ID: {}] 的学生信息: {:?}", id, student);
                        println!("准备广播给其他客户端...");
                        broadcast(&clients, &student, id);
                        message.clear(); // 清空消息缓冲区
                    }
                }
            }
            Err(e) => {
                println!("客户端 [ID: {}] 读取错误: {}", id, e);
                break;
            }
        }
    }

    // 客户端断开连接，从列表中移除
    clients.lock().unwrap().remove(&id);
    println!("客户端 [ID: {}] 已断开连接", id);
    println!("当前连接的客户端数量: {}", clients.lock().unwrap().len());
}

fn broadcast(clients: &Arc<Mutex<HashMap<usize, TcpStream>>>, student: &Student, sender_id: usize) {
    let json = serde_json::to_string(student).unwrap() + "\n";
    let mut clients = clients.lock().unwrap();
    let mut to_remove = Vec::new();
    let mut broadcast_count = 0;

    // 向所有其他客户端广播
    for (&id, stream) in clients.iter_mut() {
        if id != sender_id {
            if let Err(e) = stream.write_all(json.as_bytes()) {
                println!("向客户端 [ID: {}] 发送失败: {}", id, e);
                to_remove.push(id);
            } else {
                broadcast_count += 1;
                println!("成功广播给客户端 [ID: {}]", id);
            }
        }
    }

    println!("广播完成，成功发送给 {} 个客户端", broadcast_count);

    // 清理断开的连接
    for id in to_remove {
        clients.remove(&id);
        println!("已移除断开的客户端 [ID: {}]", id);
    }
}

