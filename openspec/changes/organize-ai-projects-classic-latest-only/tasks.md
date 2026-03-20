## 1. Data Model and Content Governance

- [x] 1.1 Add project schema fields for `isAI`, `era` (`classic` or `latest`), and `descriptionLong`.
- [x] 1.2 Add validation to reject projects missing required AI metadata when `isAI` is true.
- [x] 1.3 Backfill and curate existing project data so AI entries are tagged correctly and non-AI entries are excluded.

## 2. Filtering, Grouping, and Sorting Pipeline

- [x] 2.1 Implement AI-only filtering (`isAI = true`) before any grouping, ranking, or limits.
- [x] 2.2 Implement deterministic grouping into `classic` and `latest` sections based on `era` metadata.
- [x] 2.3 Implement section-specific ordering (impact/relevance for classic, recency for latest).
- [x] 2.4 Add handling for missing/invalid `era` values by excluding and logging flagged entries.

## 3. UI Rendering and Content Expansion

- [x] 3.1 Update the project listing UI to render two sections: "Classic AI Projects" and "Latest AI Projects".
- [x] 3.2 Increase per-section project count via configurable limits and set a larger default baseline.
- [x] 3.3 Render `descriptionLong` in project cards with responsive truncation rules for layout stability.
- [x] 3.4 Verify desktop and mobile layouts for section integrity, card consistency, and readability.

## 4. Verification, Performance, and Release

- [x] 4.1 Add tests validating non-AI exclusion, correct section assignment, and filter-before-limit behavior.
- [x] 4.2 Add tests for extended description rendering and truncation behavior.
- [x] 4.3 Run performance checks on increased project counts and optimize if regression thresholds are exceeded.
- [x] 4.4 Execute rollout plan and document rollback steps (feature flag or previous query/render path).
