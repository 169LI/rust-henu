mod models;
mod server;
mod client;

use std::env;

fn main() {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    
    // 检查参数
    if args.len() != 2 {
        println!("使用方法:");
        println!("  服务器模式: cargo run -- server");
        println!("  客户端模式: cargo run -- client");
        return;
    }

    // 根据参数选择运行模式
    match args[1].as_str() {
        "server" => {
            println!("启动服务器...");
            server::server();
        }
        "client" => {
            println!("启动客户端...");
            client::client();
        }
        _ => {
            println!("无效的模式！请使用 'server' 或 'client'");
        }
    }
}
