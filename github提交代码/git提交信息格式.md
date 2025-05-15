# 提交信息格式

提交信息通常分为三部分：**标题（Summary）**、**正文（Body）**（可选）、**脚注（Footer）**

```txt
<类型>(<范围>): <简短描述>
<空行>
<详细说明（可选）>
<空行>
<脚注（可选，例如问题编号）>
```

**标题（Summary）**：

- 格式：<类型>(<范围>): <简短描述>
- 常用类型（Type）
  - feat: 新功能（feature）。
  - fix: 修复 bug。
  - docs: 文档变更。
  - style: 代码风格调整（不影响逻辑，如格式化）。
  - refactor: 重构代码（不新增功能或修复 bug）。
  - test: 添加或修改测试。
  - chore: 杂项（例如更新依赖、配置工具）。
  - build: 构建系统或外部依赖变更。
  - ci: CI/CD 配置变更。
  - perf: 性能优化。

**正文（Body）**：

- 详细描述变更的内容、原因或背景。
- 可包含多段，使用 - 或 * 列出具体更改。
- 示例：

```
- Initialize tracing logger with console and file output 
- Support daily rolling logs in `log/` directory
```

**脚注（Footer）**：

- 包含元信息，如问题编号、关联 PR 或破坏性变更。
- 常见格式：脚注通常采用键值对或关键字形式，例如：
  - Ref: #<issue_id>：引用相关问题。
  - Closes: #<issue_id>：表示提交关闭某个问题（GitHub、GitLab 等支持）。
  - Fixes: #<issue_id>：同上，表示修复问题。
  - BREAKING CHANGE: <描述>：标记破坏性变更（影响 API 或行为）。
  - Co-authored-by: <name> <email>：记录共同作者。
  - See: <URL>：指向相关文档或 PR。

**简单示例：**

     feat(logger): add ANSI color support for console (#1)
     fix(config): resolve file encoding issue in logs (#2)
