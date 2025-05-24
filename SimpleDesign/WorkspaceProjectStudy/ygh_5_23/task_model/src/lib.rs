//! 任务管理模型库
//! 这个库提供了任务管理的核心功能，包括任务的定义、状态管理和任务列表管理。

// 模型模块，包含任务相关的结构体和枚举
pub mod model {
    // 任务状态枚举
    #[derive(Debug, Clone, PartialEq)]
    pub enum TaskStatus {
        Todo,
        InProgress,
        Done,
    }

    // 任务结构体
    #[derive(Debug, Clone)]
    pub struct Task {
        pub id: String,
        pub title: String,
        pub status: TaskStatus,
    }

    impl Task {
        // 创建新任务，返回一个新的 Task 实例，状态默认为 Todo
        pub fn new(id: String, title: String) -> Self {
            Task {
                id,
                title,
                status: TaskStatus::Todo,
            }
        }

        // 更新任务状态
        pub fn update_status(&mut self, status: TaskStatus) {
            self.status = status;
        }

        // 获取任务摘要
        pub(crate) fn get_summary(&self) -> String {
            let status_str = match self.status {
                TaskStatus::Todo => "待办",
                TaskStatus::InProgress => "进行中",
                TaskStatus::Done => "已完成",
            };
            format!("[{}] {} ({})", self.id, self.title, status_str)
        }
    }
}

// 管理器模块，包含任务管理相关的功能
pub mod manager {
    use super::model::Task;

    // 任务管理器
    #[derive(Debug, Default)]
    pub struct TaskManager {
        tasks: Vec<Task>,
    }

    impl TaskManager {
        // 创建新的任务管理器
        pub fn new() -> Self {
            TaskManager { tasks: Vec::new() }
        }

        // 添加任务
        pub fn add_task(&mut self, task: Task) {
            self.tasks.push(task);
        }

        // 列出所有任务
        pub fn list_tasks(&self) -> Vec<String> {
            self.tasks.iter().map(|task| task.get_summary()).collect()
        }
    }
}

// 重新导出常用的类型，方便外部使用
pub use model::{Task, TaskStatus};
pub use manager::TaskManager;

#[cfg(test)]
mod tests {
    use super::{Task, TaskStatus, TaskManager};

    #[test]
    fn test_task_creation() {
        let task = Task::new("1234".to_string(), "测试任务".to_string());
        assert_eq!(task.id, "1234");
        assert_eq!(task.title, "测试任务");
        assert!(matches!(task.status, TaskStatus::Todo));
    }

    #[test]
    fn test_task_status_update() {
        let mut task = Task::new("1234".to_string(), "测试任务".to_string());
        task.update_status(TaskStatus::InProgress);
        assert!(matches!(task.status, TaskStatus::InProgress));
    }

    #[test]
    fn test_task_manager() {
        let mut manager = TaskManager::new();
        let task = Task::new("1234".to_string(), "测试任务".to_string());
        manager.add_task(task);
        assert_eq!(manager.list_tasks().len(), 1);
    }
}
