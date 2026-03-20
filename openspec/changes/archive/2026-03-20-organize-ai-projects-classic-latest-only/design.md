## Context

The existing project catalog experience does not clearly distinguish between foundational AI projects and newly emerging AI projects, and currently allows non-AI entries to be mixed in. The requested change requires a clean AI-only experience, explicit sectioning (classic vs latest), larger result sets, and richer descriptions. The implementation must preserve existing page structure where possible while introducing clear content governance rules so non-AI projects cannot leak into the listing.

## Goals / Non-Goals

**Goals:**
- Introduce a deterministic categorization model with two AI sections: classic and latest.
- Enforce an AI-only filtering rule at the data-selection layer.
- Expand list sizing and content shape so each project can render a longer, more informative description.
- Keep rendering behavior predictable across desktop and mobile layouts.

**Non-Goals:**
- Redesign unrelated site sections outside the AI project listing area.
- Introduce automated ML-based classification in this phase; categorization remains rule/tag driven.
- Backfill every historical project in one release if source metadata is incomplete.

## Decisions

1. Use explicit metadata fields for selection and grouping.
   - Decision: Each project record includes `isAI` (boolean), `era` (`classic` or `latest`), and `descriptionLong` (extended text).
   - Rationale: Keeps filtering and rendering straightforward, testable, and audit-friendly.
   - Alternative considered: Infer AI relevance from keywords in title/description; rejected due to false positives and governance risk.

2. Apply AI-only filtering before any sorting, limiting, or rendering.
   - Decision: Data pipeline first filters `isAI === true`, then groups by `era`, then sorts and limits per section.
   - Rationale: Prevents non-AI items from entering downstream UI paths.
   - Alternative considered: Filter in the UI layer only; rejected because upstream leakage can still affect counts and UX.

3. Define section-specific ordering rules.
   - Decision: `classic` sorted by long-term impact/relevance rank; `latest` sorted by recency (publish/update time).
   - Rationale: Aligns with user intent: foundational exploration vs recent discovery.
   - Alternative considered: Single global ranking for both sections; rejected because it blurs section purpose.

4. Expand description rendering with safe truncation.
   - Decision: Render `descriptionLong` with a higher character budget and ellipsis fallback when overflow occurs.
   - Rationale: Provides richer context while preserving visual stability.
   - Alternative considered: Fixed short summary only; rejected because it does not meet the requirement for richer introductions.

## Risks / Trade-offs

- [Incomplete metadata for existing projects] -> Mitigation: add validation checks and fallback review list for uncategorized items.
- [Misclassification of AI relevance by content editors] -> Mitigation: enforce required `isAI` and `era` fields in content schema and CI validation.
- [Long descriptions may cause uneven card heights] -> Mitigation: clamp text lines with responsive thresholds and provide consistent card layout rules.
- [Larger project count may affect page performance] -> Mitigation: lazy-load section assets and cap per-section counts with configurable limits.

## Migration Plan

1. Extend project content schema with `isAI`, `era`, and `descriptionLong`.
2. Backfill metadata for currently visible projects, prioritizing AI projects first.
3. Implement filtering/grouping pipeline and section rendering updates behind a feature flag if available.
4. Validate that non-AI projects are excluded and both sections populate with expanded AI entries.
5. Roll out to production; monitor rendering/performance metrics and content accuracy.
6. Rollback strategy: disable feature flag or revert to previous listing query/render path if severe regressions occur.

## Open Questions

- What exact threshold defines "classic" vs "latest" when a project could fit both (time-based cutoff, editorial list, or hybrid)?
- What minimum and maximum project count per section should be the default for launch?
- Should extended descriptions support markdown-rich formatting or remain plain text for consistency?
