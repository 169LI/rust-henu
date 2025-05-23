use task_model::{Task, TaskStatus, TaskManager};
use utils::id_gen;

fn main() {
    // 创建任务管理器
    let mut manager = TaskManager::new();

    // 创建一些示例任务
    let task1 = Task::new(id_gen::generate_id(), "完成项目文档".to_string());
    let task2 = Task::new(id_gen::generate_id(), "代码审查".to_string());
    let task3 = Task::new(id_gen::generate_id(), "单元测试".to_string());

    // 添加任务
    manager.add_task(task1);
    manager.add_task(task2);
    manager.add_task(task3);

    // 列出所有任务
    println!("任务列表：");
    for task in manager.list_tasks() {
        println!("{}", task);
    }
} 