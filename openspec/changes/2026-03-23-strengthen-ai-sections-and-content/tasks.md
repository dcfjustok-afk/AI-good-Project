## 1. AI-Only Hardening

- [ ] 1.1 Review current AI relevance heuristic and define stricter exclusion rules for weak-signal or non-AI repositories.
- [ ] 1.2 Apply the stricter AI-only gate before ranking, grouping, pagination, and per-section limits.
- [ ] 1.3 Add verification that non-AI repositories cannot appear in classic or latest sections.

## 2. AI Section Expansion

- [ ] 2.1 Expand upstream GitHub AI search breadth with additional AI-focused queries and/or higher per-query limits.
- [ ] 2.2 Increase the default number of projects shown in both classic and latest sections.
- [ ] 2.3 Validate that section ordering still matches section intent after inventory growth.

## 3. Richer Project Introductions

- [ ] 3.1 Expand `descriptionLong` generation so each project carries more context than the current baseline.
- [ ] 3.2 Verify project cards and any section previews remain readable with the longer content.
- [ ] 3.3 Add fallback behavior for repositories that still have sparse upstream metadata.

## 4. Validation and Release

- [ ] 4.1 Run sync against real AI data and confirm both sections populate with AI-only projects.
- [ ] 4.2 Verify build, Rust checks, and homepage behavior after section-size and description changes.
- [ ] 4.3 Commit changes atomically and push after each completed implementation slice.