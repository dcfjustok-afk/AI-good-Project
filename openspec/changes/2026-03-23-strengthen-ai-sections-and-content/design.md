## Context

The repository already contains a first-pass implementation for AI-only filtering, classic/latest segmentation, and richer card content. The new request tightens the product contract: classic and latest sections must remain, non-AI projects must never leak into the experience, the available AI pool should be larger, and descriptions should be more detailed. This change therefore focuses on hardening selection and expanding content depth rather than inventing a new browsing model.

## Goals / Non-Goals

**Goals**
- Enforce stricter AI-only project eligibility.
- Expand the classic/latest AI sections with more projects.
- Increase description depth without breaking layout stability.
- Keep homepage browsing centered on the two AI sections.

**Non-Goals**
- Adding non-AI categories or mixed technology sections.
- Replacing the existing section model with a single flat feed.
- Introducing manual editorial workflows outside the current sync pipeline.

## Decisions

1. Strengthen AI relevance at the data pipeline instead of only at rendering time.
   - Decision: filtering remains upstream, but the eligibility heuristic becomes stricter and excludes weak-signal repositories.
   - Rationale: prevents non-AI entries from consuming quota in either section.

2. Increase usable AI supply by expanding upstream search breadth.
   - Decision: broaden AI search keywords and per-query result count while preserving deduplication.
   - Rationale: more valid AI repositories are needed before larger section limits are meaningful.

3. Preserve classic/latest as the homepage’s primary content frame.
   - Decision: homepage should continue surfacing classic and latest AI sections as the first discovery experience.
   - Rationale: matches current product direction and the user’s explicit request.

4. Treat long descriptions as a structured synthesis field, not raw unbounded text.
   - Decision: descriptionLong should combine repository description, highlights, use cases, and frontend/product relevance into a denser but still clamp-safe field.
   - Rationale: richer context is needed, but uncontrolled text growth would destabilize the UI.

## Risks / Trade-offs

- Stricter AI filtering may reduce available inventory if search breadth is not increased in parallel.
- Larger sections may surface more low-quality projects unless ranking is recalibrated after filtering.
- Longer descriptions can create visual imbalance unless line-clamping and spacing remain consistent.

## Migration Plan

1. Tighten AI eligibility heuristics in the sync pipeline.
2. Expand AI search coverage and section limits.
3. Regenerate or backfill richer descriptionLong content during sync.
4. Validate homepage section counts, exclusion behavior, and card readability.

## Open Questions

- What should the default visible count be for classic and latest sections after expansion?
- Should the “latest” section prioritize `updated_at`, repository age, or a hybrid freshness score?