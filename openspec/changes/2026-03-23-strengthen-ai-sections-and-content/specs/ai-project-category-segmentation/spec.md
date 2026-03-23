## MODIFIED Requirements

### Requirement: AI project listing SHALL be segmented into classic and latest sections
The system SHALL keep two explicit homepage sections for AI projects named “经典 AI 项目” and “最新 AI 项目”, and these sections SHALL be the primary AI browsing surface.

#### Scenario: Homepage loads AI inventory successfully
- **WHEN** the system loads enough valid AI projects
- **THEN** it renders both sections with distinct project groups and does not merge them into a single mixed feed

#### Scenario: Section inventory is insufficient
- **WHEN** one section has fewer valid AI projects than the configured target count
- **THEN** the system renders all eligible projects for that section without backfilling from the other section or from non-AI entries