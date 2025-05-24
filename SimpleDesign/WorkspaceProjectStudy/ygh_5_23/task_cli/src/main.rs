use std::io::{self, Write};
use task_model::{Task, TaskManager};
use utils::id_gen;

fn main() {
    let mut task_manager = TaskManager::new();
    
    loop {
        println!("\n任务管理系统");
        println!("1. 添加新任务");
        println!("2. 列出所有任务");
        println!("3. 退出程序");
        print!("请选择操作 (1-3): ");
        //立即输出
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                print!("请输入任务标题: ");
                io::stdout().flush().unwrap();
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                let title = title.trim().to_string();

                let id = id_gen::generate_id();
                let task = Task::new(id, title);
                task_manager.add_task(task);
                println!("任务已添加！");
            }
            "2" => {
                let tasks = task_manager.list_tasks();
                if tasks.is_empty() {
                    println!("当前没有任务！");
                } else {
                    println!("\n任务列表：");
                    for task in tasks {
                        println!("{}", task);
                    }
                }
            }
            "3" => {
                println!("感谢使用，再见！");
                break;
            }
            _ => println!("无效的选择，请重试！"),
        }
    }
}
