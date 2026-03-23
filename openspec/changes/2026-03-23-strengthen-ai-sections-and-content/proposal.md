## Why

The current experience already includes classic and latest AI sections, but the business rule is still not explicit enough for strict AI-only curation, larger section sizes, and richer project introductions. The user requirement is narrower and stronger: only AI-related projects should appear, classic and latest columns should be first-class discovery surfaces, and every card should expose more useful context instead of short summaries.

## What Changes

- Strengthen the AI-only selection rule so ambiguous or non-AI repositories are excluded before any ranking or sectioning.
- Keep two explicit AI sections, one for classic AI projects and one for latest AI projects, and make them the primary browsing mode.
- Increase the number of projects in each AI section beyond the current baseline.
- Expand project introduction content so each project shows a richer long description built from repository metadata and structured summary signals.

## Capabilities

### Modified Capabilities
- `ai-only-project-filtering`: tighten AI eligibility and reject non-AI or weak-signal repositories before listing.
- `ai-project-category-segmentation`: make classic/latest AI sections the primary listing contract with stronger section integrity.
- `enhanced-ai-project-content`: increase per-section counts and require richer long-form introductions.

## Impact

- Affects GitHub fetch/query strategy, repository eligibility rules, section-level limits, and homepage section defaults.
- May require stricter AI relevance heuristics and more GitHub search keywords to increase the usable AI-only pool.
- Requires UI copy and card layout checks to ensure longer descriptions remain readable.