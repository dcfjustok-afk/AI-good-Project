## MODIFIED Requirements

### Requirement: Listing SHALL include only AI-related projects
The system SHALL include only repositories that pass a strict AI relevance gate in the AI listing experience.

#### Scenario: Repository has weak or ambiguous AI signals
- **WHEN** a repository does not clearly indicate AI relevance through keywords, topics, or summary classification
- **THEN** the system excludes it from all AI sections instead of allowing it to compete for ranking slots

#### Scenario: Non-AI repository appears in upstream results
- **WHEN** GitHub search returns a repository unrelated to AI
- **THEN** the repository is discarded before any classic/latest grouping or count limit is applied

### Requirement: Filtering SHALL occur before ranking and limits
The system MUST apply the strict AI-only filter before any grouping, sorting, pagination, or per-section count limits.

#### Scenario: Expanded section limit is enabled
- **WHEN** section limits increase to show more projects
- **THEN** the additional slots are filled only from the filtered AI subset