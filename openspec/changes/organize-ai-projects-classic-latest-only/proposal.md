## Why

Current project listing mixes AI and non-AI items, and does not clearly separate foundational "classic AI" projects from recently launched AI projects. This makes discovery harder for users who only want AI-related content and expect richer project context.

## What Changes

- Add dedicated sections for AI projects: one for classic AI projects and one for latest AI projects.
- Restrict displayed projects to AI-related items only; exclude all non-AI projects from this experience.
- Increase the number of AI projects shown in each section.
- Expand project descriptions so each entry includes more useful context (scope, highlights, and relevance).

## Capabilities

### New Capabilities
- `ai-project-category-segmentation`: Define and present AI projects in two explicit categories: classic and latest.
- `ai-only-project-filtering`: Enforce a content rule that only AI-related projects are eligible for display.
- `enhanced-ai-project-content`: Support larger AI project collections and richer per-project descriptions.

### Modified Capabilities
- None.

## Impact

- Affects project selection and rendering logic for the project listing experience.
- Requires data/schema updates for category tagging, AI relevance filtering, and extended description fields.
- May require content curation workflow updates to classify projects and validate AI-only inclusion.
