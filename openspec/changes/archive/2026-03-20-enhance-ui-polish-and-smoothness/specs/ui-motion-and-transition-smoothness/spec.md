## ADDED Requirements

### Requirement: 核心交互 MUST 提供语义化过渡动效
系统 MUST 在页面进入、筛选切换、卡片悬浮与列表刷新等关键交互中提供语义明确的过渡动效，并保持时长与节奏一致性。

#### Scenario: 用户切换筛选条件
- **WHEN** 用户在筛选区变更排序或栏目条件
- **THEN** 列表更新过程展示平滑过渡且无突兀闪烁

### Requirement: 动效系统 SHALL 支持可降级策略
系统 SHALL 在检测到用户偏好减少动态效果或设备性能受限时，自动降级为轻量过渡以保证流畅性。

#### Scenario: 用户系统开启 reduced motion
- **WHEN** 前端检测到 reduced motion 偏好
- **THEN** 页面使用低动效方案并保留必要状态反馈
