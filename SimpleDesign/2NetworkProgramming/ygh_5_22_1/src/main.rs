mod models;
mod server;
mod client;

use std::env;

fn main() {
    //获取命令行参数
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("请指定运行模式: server 或 client");
        return;
    }

    // 直接在model.rs中修改测试数据

    //根据参数运行
    match args[1].as_str() {
        "server" => server::run_server(),
        "client" => client::run_client(),
        _ => println!("无效的模式，请使用 server 或 client"),
    }
}
