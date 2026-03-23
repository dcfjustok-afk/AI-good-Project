## Purpose
Define a reusable visual token system that improves consistency, readability, and hierarchy across core UI surfaces.

## Requirements

### Requirement: 界面视觉系统 MUST 统一并可复用
系统 MUST 通过统一的视觉 token（颜色、间距、圆角、阴影、层级）驱动核心页面与组件的样式表达，避免同类元素出现明显风格漂移。

#### Scenario: 首页核心组件使用统一 token
- **WHEN** 首页渲染头部、筛选区、卡片区和状态区
- **THEN** 各区域使用同一套视觉 token，呈现一致的层级与质感

### Requirement: 视觉优化 SHALL 提升可读性与信息层次
系统 SHALL 在文本层级、间距节奏和对比度上提升可读性，确保关键信息在首屏与列表中易于扫描。

#### Scenario: 项目卡片展示关键信息
- **WHEN** 用户浏览项目列表卡片
- **THEN** 标题、描述、标签和辅助信息具备清晰层级且不互相抢占视觉焦点
