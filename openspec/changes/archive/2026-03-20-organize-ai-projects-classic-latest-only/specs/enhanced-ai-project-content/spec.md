## ADDED Requirements

### Requirement: System SHALL support expanded AI project counts per section
The system SHALL support configurable project counts for each AI section and SHALL default to a larger count than the previous baseline.

#### Scenario: Configuration defines per-section count
- **WHEN** section count configuration is set
- **THEN** the system renders up to the configured number of AI projects per section

#### Scenario: Available projects are fewer than configured count
- **WHEN** a section has fewer AI projects than the configured count
- **THEN** the system renders all available projects without placeholders that imply missing non-AI content

### Requirement: Project entries SHALL include richer descriptions
Each displayed AI project MUST include an extended description field with substantially more context than a short title summary.

#### Scenario: Extended description is present
- **WHEN** an AI project includes extended descriptive content
- **THEN** the UI renders the longer description for that project entry

#### Scenario: Extended description exceeds layout budget
- **WHEN** the description length exceeds the rendering threshold
- **THEN** the system truncates safely with visual continuity and without dropping project identity information
