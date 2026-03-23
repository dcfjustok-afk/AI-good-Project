## Purpose
Standardize status feedback patterns so loading, empty, error, and success states are clear, consistent, and actionable.

## Requirements

### Requirement: 状态反馈 MUST 统一为标准化组件表达
系统 MUST 对 loading、empty、error、success 四类状态使用统一结构与视觉规范，保证跨页面反馈一致。

#### Scenario: 列表请求中与请求失败
- **WHEN** 用户进入页面触发列表请求并发生不同状态
- **THEN** 页面分别展示标准化 loading 和 error 反馈且语义清晰

### Requirement: 状态文案 SHALL 明确且可操作
系统 SHALL 为状态反馈提供简洁明确的中文文案，并在可恢复场景下提供清晰的下一步操作入口。

#### Scenario: 数据为空场景
- **WHEN** 筛选结果为空
- **THEN** 页面展示空态说明并提供重置筛选或切换条件的操作入口
