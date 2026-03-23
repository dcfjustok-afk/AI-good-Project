## Purpose
Improve responsive behavior so desktop and mobile layouts remain consistent, readable, and stable across breakpoint changes.

## Requirements

### Requirement: 多端布局 MUST 保持结构稳定与阅读舒适
系统 MUST 在桌面端与移动端维持核心信息结构一致，并通过断点策略优化间距、列数与组件尺寸。

#### Scenario: 移动端访问首页
- **WHEN** 视口宽度处于移动端范围
- **THEN** 布局自动调整为更高可读性版本，避免信息拥挤与控件重叠

### Requirement: 布局切换 SHALL 降低视觉跳动
系统 SHALL 在断点变化与内容刷新时控制高度波动和重排冲击，减少用户感知到的“抖动”。

#### Scenario: 用户旋转设备或调整窗口
- **WHEN** 视口从一个断点切换到另一个断点
- **THEN** 页面以平滑方式完成重排且核心操作入口保持可见
