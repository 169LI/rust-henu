# 任务管理系统

这是一个使用 Rust 语言开发的命令行任务管理系统。该系统采用模块化设计，包含多个组件，展示了 Rust 的模块化编程和包管理特性。

## 项目结构

```
task_management/
├── Cargo.toml
├── task_model/          # 核心任务管理模型
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
├── utils/              # 通用工具函数
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
├── task_cli/          # 命令行界面
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── examples/          # 使用示例
│   └── basic_usage.rs
└── tests/            # 测试模块
    ├── task_model_tests.rs
    └── utils_tests.rs
```

## 功能特点

- 任务管理：创建、更新和查看任务
- 状态追踪：支持待办、进行中和已完成三种状态
- 随机ID生成：为每个任务生成唯一标识符
- 日期格式化：支持日期显示和格式化
- 命令行界面：提供友好的用户交互

## 构建和运行

### 环境要求

- Rust 1.70.0 或更高版本
- Cargo 包管理器

### 构建项目

```bash
cargo build
```

### 运行程序

运行主程序：
```bash
cargo run --bin task_cli
```

运行示例：
```bash
cargo run --example basic_usage
```

### 运行测试

运行所有测试：
```bash
cargo test
```

运行特定测试：
```bash
cargo test test_task_creation  # 运行特定测试函数
cargo test task_model          # 运行特定模块的测试
```

## 使用说明

1. 启动程序后，会显示主菜单
2. 选择相应的数字进行操作：
   - 1：添加新任务
   - 2：查看任务列表
   - 3：退出程序

## 开发说明

- 使用 Rust 2021 版本
- 遵循 Rust 标准编码规范
- 包含完整的文档注释
- 包含单元测试和集成测试
- 提供使用示例

## 模块说明

### task_model

核心任务管理模型，提供：
- 任务结构定义
- 状态管理
- 任务列表管理

### utils

通用工具函数库，提供：
- ID生成器
- 日期格式化工具

### task_cli

命令行界面，提供：
- 用户交互
- 任务管理操作
- 数据展示

### examples

使用示例，展示：
- 基本功能使用
- API调用方式
- 最佳实践

### tests

测试模块，包含：
- 单元测试
- 集成测试
- 功能验证 