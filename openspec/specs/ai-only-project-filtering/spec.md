## Purpose
Ensure AI project listings include only explicitly AI-related entries before any downstream ranking or limits are applied.

## Requirements

### Requirement: Listing SHALL include only AI-related projects
The system SHALL include only projects explicitly marked as AI-related in the project listing experience.

#### Scenario: Mixed AI and non-AI source data
- **WHEN** the source dataset contains both AI and non-AI projects
- **THEN** only projects marked with `isAI = true` are eligible for display

#### Scenario: Non-AI project appears in source updates
- **WHEN** a project is added or updated with `isAI = false`
- **THEN** the project is excluded from all AI listing sections

### Requirement: Filtering SHALL occur before ranking and limits
The system MUST apply AI filtering before any section grouping, sorting, or per-section count limits.

#### Scenario: Section limit is reached
- **WHEN** more than the maximum number of projects exist after filtering
- **THEN** the system applies sorting and limits only to the filtered AI subset

#### Scenario: Upstream query returns non-AI entries first
- **WHEN** upstream ordering includes non-AI items in top positions
- **THEN** the final rendered output contains no non-AI projects
