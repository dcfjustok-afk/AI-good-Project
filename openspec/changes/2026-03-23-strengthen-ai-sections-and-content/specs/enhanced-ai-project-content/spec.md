## MODIFIED Requirements

### Requirement: System SHALL support expanded AI project counts per section
The system SHALL increase the default classic/latest section counts beyond the current baseline and SHALL source the additional entries from the AI-only inventory.

#### Scenario: More AI projects are available after search expansion
- **WHEN** the sync pipeline finds more eligible AI projects
- **THEN** each section renders more projects than before up to the configured section limit

### Requirement: Project entries SHALL include richer descriptions
Each displayed AI project MUST include a longer introduction field with meaningfully more context than the current short summary baseline.

#### Scenario: Structured summary fields are available
- **WHEN** summary, highlights, use cases, and relevance signals are available
- **THEN** the system composes a denser `descriptionLong` field suitable for card display and section browsing

#### Scenario: Upstream metadata is sparse
- **WHEN** repository metadata is too thin for a full rich introduction
- **THEN** the system builds the longest safe fallback it can from available description, category, and topic signals rather than reverting to a minimal one-line summary