---
type: Research Report
title: Reference authoring style for OKF
description: Prefer document-local keyed claim citations, retain useful source registers, and promote reused evidence into reference concepts selectively.
tags: [domain/okf, concern/evidence, concern/authoring]
status: draft
generated: { by: process:cursor, at: 2026-09-11T08:07:00Z }
stale_after: 2026-12-11
authority: exploratory
owners: [human:nils]
sources:
  - id: spec
    resource: https://raw.githubusercontent.com/GoogleCloudPlatform/knowledge-catalog/main/okf/SPEC.md
    title: OKF v0.2 specification
    author: organization:google-cloud
  - id: load
    resource: ../../../okf/src/load.rs
    title: Source and footnote matching
  - id: graph
    resource: ../../../okf/src/graph.rs
    title: Markdown link and heading resolution
  - id: markdown
    resource: ../../../okf/src/markdown.rs
    title: Markdown extraction
  - id: validate
    resource: ../../../okf/src/validate.rs
    title: Source validation and repository provenance
  - id: peek
    resource: ../../plans/okmate/peek-and-tabs.md
    title: Viewer peek previews and document tabs
    author: process:cursor
    last_modified: 2026-09-11
---

# Reference authoring style for OKF

## Recommendation and scope

Use a matching source entry and a short footnote at the supported claim as
the default. Prefer descriptive, stable keys for new material. Keep a shared
source register when it contributes selection rationale, evidence limitations,
or research history. Create individual reference concepts when they deserve
independent maintenance. These are proposed authoring preferences, not an
approved project policy or a new format requirement.

This investigation concerns reference structure, not the accuracy of the
underlying agentic-development research. The motivating pattern is a report
linking to a shared register's `#s21` section, which in turn cites an external
publication. No external publication claims were reverified here.

## What is already specified

OKF v0.2 defines keyed footnotes for per-claim attribution: their labels join
to `sources[].id`. Source IDs are stable keys, not list positions. Provenance
is optional; each supplied source entry requires `resource`. The specification
also permits reference concepts and conventionally places mirrored material
under `references/`. It does not require a shared bibliography, numeric keys,
or one file per publication.[^spec]

Thus the citation mechanism is specified; the choice of bibliography layout
remains an authoring decision.

## What the current engine actually resolves

Implementation inspected at okmate commit
`d616cd9470b0ac3e757e198730fe9d0f351aac6b` on September 11 2026.

- A link such as `register.md#s21` targets a Markdown heading. It does not
  look up `sources[].id`. Heading IDs and footnote labels are collected
  separately.[^graph][^markdown]
- Footnote/source matching happens within each concept. Missing matching
  source IDs or body definitions are errors (`OKF4001`, `OKF4003`); an unused
  source ID produces warning `OKF4002`. These checks run outside the profile
  branch, so they are not exclusive to Strict.[^load]
- Duplicate source IDs are rejected within a document. Reusing a key in
  another document does not establish shared identity.[^validate]
- Graph edges are built from body links, not a traversal of source metadata.
  A source entry alone therefore does not provide a navigable graph edge in
  this implementation. Include a normal link in its footnote when graph
  navigation matters.[^graph]
- Repository provenance treats a relative source resource as a filesystem
  path and does not strip a fragment. For local references, keep the path in
  metadata and put any heading fragment in the body link. Bundle-root paths
  are supported by the body graph resolver but skipped by this repository
  provenance pass.[^validate][^graph]

Validation checks structural attribution, not whether a citation entails the
claim or whether an external URL is still accurate.[^load][^validate]

## Options

| Pattern | Useful when | Cost or limitation | Proposed use |
| --- | --- | --- | --- |
| Direct URL in prose | Navigation or incidental further reading | No keyed claim attribution | Keep for navigation; add a keyed footnote for evidence |
| Local source entry plus claim footnote | A report derives claims from a publication | Some metadata repeats across reports | Default |
| Shared register with numbered headings | A bounded research project needs a corpus and reading history | Extra hop; opaque labels; a link alone does not give the consuming claim a source-key join | Retain as a research artifact, alongside precise citations |
| One reference concept per publication | Evidence is reused and has a maintained summary, caveats, or review history | More files, indexes, and lifecycle work | Promote selectively |
| Reference-style Markdown links (`[label][key]`) | Repeated navigation links need one URL definition | A link definition is not a keyed evidence footnote | Readability tool, not a citation replacement |
| BibTeX/CSL or a custom shared-source registry | An established scholarly workflow needs import/export | Adds tooling and a separate identity/resolution contract | Optional integration; materialize portable citations on export |

The costs and preferences in this table are design judgments. In particular,
do not infer that a custom source-registry resolver already exists in OKMate.

## Recommended authoring conventions

1. Cite the material actually used. If a report relies on a curated summary,
   cite that summary; if it derives claims directly from the publication,
   cite the publication. Do not imply primary-source review merely by copying
   a primary URL from another report.
2. Use keys such as `steinberger-inference-speed-2025` for new sources. A key
   can contain a number, but must not change when the bibliography is sorted.
   Existing `s21` keys are acceptable if treated as immutable identifiers.
3. Place the marker at the claim or a clearly bounded group of claims. A
   single register citation in a method section is too coarse for a report
   that attributes many unrelated empirical claims.
4. Keep bibliographic metadata in the source entry and footnote prose short:
   a readable link, useful locator, and any claim-specific caveat. Avoid a
   third copy of the entire citation in a body bibliography unless that body
   is deliberately an annotated source register.
5. Distinguish publication date, source modification time, retrieval date,
   and verification. Record retrieval context in prose or a documented
   extension; do not use a formatting change as evidence of a new source check.
6. A promoted reference concept should contain a useful bounded summary,
   evidence class, limitations, and original-source citation. Give it a
   descriptive path and its own lifecycle. A URL wrapper alone rarely merits
   the additional file.
7. Preserve published heading anchors. For an existing `S21` section, retain
   that heading and improve link labels or the text beneath it. Renaming it
   to a descriptive heading changes the anchor; migrate inbound links if
   intentionally making that change.

## Examples

Illustrative source-entry excerpt, to be used inside a complete record:

```yaml
sources:
  - id: study-2026
    resource: https://example.org/study
    title: Example study
```

```markdown
The authors report the bounded outcome.[^study-2026]

[^study-2026]: [Example study](https://example.org/study), results section;
    an author-reported outcome.
```

When the actual evidence is a maintained local reference concept, use a
record-relative path in the source entry and a body link for navigation:

```yaml
sources:
  - id: curated-study
    resource: ../../references/example-study.md
    title: Curated example study
```

```markdown
The curated account identifies this limitation.[^curated-study]

[^curated-study]: [Curated example study](/references/example-study.md#limitations).
```

These examples are alternative excerpts, not files to add verbatim. The
target concept and heading must exist for the second form.

## Applying this to an existing numbered register

Keep the register and its anchors initially. Improve the consuming reports
first: replace generic source links with claim-local keyed citations to the
material actually consulted. A register-section link can remain in the
footnote as additional research context. If the register itself was the
material used, cite it honestly and link the relevant section.

For new reference material, prefer descriptive keys. Do not mass-rename old
numeric IDs solely for appearance. Extract a source into a reference concept
only when repeated use or independent curation justifies it; retain the old
register section as a pointer so existing links still work.

No register migration or citation-resolution code change is part of this
research record. General OKF guidance belongs here; domain evidence stays in
its owning knowledge bundle. Live-preview hover of keyed footnotes and
register heading hashes is a separate okmate plan.[^peek]

## Evidence

[^spec]: [OKF v0.2 specification](https://raw.githubusercontent.com/GoogleCloudPlatform/knowledge-catalog/main/okf/SPEC.md), sections 4.2, 5.1, and 6; read September 11 2026.
[^load]: `okf/src/load.rs`, per-concept source/footnote matching.
[^graph]: `okf/src/graph.rs`, `resolve_graph` and bundle-path resolution.
[^markdown]: `okf/src/markdown.rs`, heading, link, and footnote extraction.
[^validate]: `okf/src/validate.rs`, `collect_source_ids`, `validate_lifecycle_and_sources_with`, and `repository_source_path`.
[^peek]: [Viewer peek previews and document tabs](/plans/okmate/peek-and-tabs.md).
