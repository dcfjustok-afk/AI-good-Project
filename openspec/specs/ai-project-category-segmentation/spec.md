## Purpose
Define deterministic segmentation of AI projects into classic and latest sections so listings are consistent and maintainable.

## Requirements

### Requirement: AI project listing SHALL be segmented into classic and latest sections
The system SHALL render AI projects in two distinct sections named "Classic AI Projects" and "Latest AI Projects".

#### Scenario: Both categories are available
- **WHEN** the system loads AI projects with category metadata for classic and latest items
- **THEN** it renders two separate sections with projects grouped under their matching category labels

#### Scenario: One category has no projects
- **WHEN** no AI projects are available for one category
- **THEN** the system still renders the available category section and does not place projects into the wrong section

### Requirement: Project category assignment SHALL be deterministic
The system MUST assign each AI project to exactly one category (`classic` or `latest`) based on explicit project metadata.

#### Scenario: Project includes valid era metadata
- **WHEN** an AI project has `era` set to `classic` or `latest`
- **THEN** the system assigns the project only to the specified section

#### Scenario: Project has missing or invalid era metadata
- **WHEN** an AI project lacks valid category metadata
- **THEN** the system excludes it from both rendered sections and records it for content correction
