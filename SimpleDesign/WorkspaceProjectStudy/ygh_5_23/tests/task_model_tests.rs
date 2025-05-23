use task_model::{Task, TaskStatus, TaskManager};

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