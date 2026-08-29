# okf

Portable, UI-neutral engine for Open Knowledge Format (OKF) bundles.

`okf` provides deterministic parsing, schema validation, graph resolution, lexical search, retrieval benchmarks, and artifact generation for OKF knowledge repositories.

## Scope & Dependencies

`okf` is completely UI-neutral. It can be consumed by third-party Rust applications or external tools without pulling in HTML, HTTP, or desktop runtime.

Dependencies:
- `comrak`: Inert CommonMark markdown body parsing.
- `syntect`: Class-based syntax highlighting for fenced code in `article_html`.
- `yaml-rust`: YAML frontmatter extraction with lossless preservation of custom keys.
- `serde`, `serde_json`, `toml`: Data serialization and benchmark parsing.
- `sha2`: Cryptographic digest calculations.
- `anyhow`: Operation-level I/O and usage errors (`not a directory`, missing inspect id). Diagnostic structs stay separate.

## Public surface

Operations: `load`, `load_timed`, `load_with_cache`, `check`, `inspect`, `inspect_filtered`, `search`, `build`, `build_artifacts`, `benchmark_retrieval`.

AST and reports: `Bundle`, `Concept`, `Index`, `Log`, `Edge`, `Heading`, `HeadingSection`, `Link`, `Span`, `Profile`, `LoadOptions`, `LoadResult`, `LoadTimings`, `InspectKind`, `KnowledgeFilter`, `TrustTier`, `CheckReport`, `BuildSummary`, `Diagnostic`, `Severity`, `SourceLocation`, retrieval report types.

Helpers callers already need: `string_field`, `metadata_string_array`, `latest_human_verification`, `classify_concept_action`, `ActionKind`, `ConceptAction`, `ParseCache`, `PARSE_CACHE_VERSION`, `published_href`, `resolve_preview_path`, `PreviewTarget`, `concept_trust_tier`, `concept_is_stale`.

Parse, git, and civil-date internals stay crate-private.

## Core Features

- **Multi-Profile Validation**: `Profile::Base` (portable OKF specification) and `Profile::Strict` (evidence, verification, and owners).
- **Load timings**: `load_timed` returns ordinary `Duration` breakdowns (`discover`, `parse`, `graph`, and `provenance` when git provenance runs) beside the `Bundle`. `LoadOptions` selects the profile and whether provenance runs. `ParseCache` reuses unchanged documents across loads, including from a caller-provided directory via `load_dir` / `save_dir`. `okf` does not depend on CLI snapshot types and does not choose config paths.
- **Graph Resolution**: Strict and fuzzy concept ID matching, fragment checking, and directed edge construction. Authored `/path.md` links are bundle-root; `article_html` rewrites in-bundle Markdown hrefs to published `/{id}/` routes while `concept.links` keep the source URLs. `okf:` hrefs are classified like `mailto:` (not intra-bundle paths, not OKF3001).
- **Search & Chunking**: Semantic search indexing by metadata and headings with BM25/lexical matching.
- **Retrieval Benchmarking**: Automated evaluation against test questions with hit rate and MRR metrics.
- **Deterministic Build**: Emits `catalog.json`, `search.json`, `validation.json`, and `llms.txt`.
- **Preview path resolution**: Resolves a bundle directory, root `index.md`, or concept `.md` file to a bundle root and open URL.
