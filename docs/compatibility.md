# OKF reader compatibility inventory

This inventory documents **current** `okf` / `okmate` diagnostics. It is not
a proposal to change them. Phase 3 of the heterogeneous-authoring plan may
split format, evidence, and local style; until then, treat the table as the
behavior authors will see.

Executable fixtures live in [`okf/tests/fixtures/compatibility/`](../okf/tests/fixtures/compatibility/).
They are small synthetic trees. Third-party repositories are not golden
fixtures.

Rule ownership:

- **OKF format** — portable parse and citation rules the engine should
  enforce for any producer.
- **Okmate reader limit** — current `base` rejects or ignores something a
  tolerant interchange reader would accept. Honest gap, not a spec proof.
- **Okmate Strict** — owners, evidence, and this product’s type/tag list.
- **Okmate isolation** — keep record bodies inert and unsafe HTML out of
  rendered output.

| Fixture | Owner | Code | Level | Profile | What you will see |
| --- | --- | --- | --- | --- | --- |
| `malformed-yaml` | OKF format (`okf` parse) | `OKF1003` | error | base and strict | Broken YAML is a parse failure. The Markdown body may still be captured. |
| `citation-missing-definition` | OKF format (citations) | `OKF4003` | error | base and strict | A `[^id]` footnote with no definition in the body. |
| `citation-unused-source` | OKF format (citations) | `OKF4002` | warning | base and strict | A `sources[].id` never used by a body footnote. |
| `peer-root-extension` | Okmate reader limit (`okf` load) | `OKF1011` | error | base and strict | Root `index.md` may only contain `okf_version`. Peer tools that add profile or schema keys fail here. This is not an OKF prohibition of every extra root key. |
| `v0.1-version` | Okmate reader limit | `OKF1012` | error | base and strict | Only string `0.2` is accepted. v0.1 declarations fail. |
| `verified-mapping` | Okmate reader limit | `OKF1010` | error | base and strict | `verified` must be a **list** of `{by, at}` mappings. A single mapping is not normalized. |
| `stale-after-timestamp` | Okmate reader limit | `OKF1006` | error | base and strict | `stale_after` must be `YYYY-MM-DD`, not an RFC 3339 timestamp. |
| `unknown-metadata` | OKF format (optional metadata) | `OKF2001` | warning | base and strict | Unknown fields are **preserved** and warned. They are not stripped. |
| `unknown-type` | Okmate Strict | `OKF2002` | warning | strict only | `type: Workflow` (or any type outside ten product names) is valid on `base` and warns on `strict`. |
| `missing-domain-tag` | Okmate Strict | `OKF2004` | error | strict only | Strict requires at least one `domain/*` tag and only `domain/`, `integration/`, `concern/`, `audience/` prefixes (`OKF2005`). |
| `html-comment` | Okmate isolation | `OKF2009` | error | base and strict | HTML comments are diagnosed as raw HTML, same as active markup. Rendering still excludes unsafe HTML. |
| `rocdown-declaration` | Okmate isolation | `OKF2007` | error | base and strict | `@page` / `@render` and other Rocdown declarations are forbidden in knowledge bodies. |

## How to read a check report today

- **Errors** block a clean `check` and `build`.
- **Warnings** do not fail `check` unless you treat them as errors in review.
- `base` is the portable profile, but it is **not** yet a complete tolerant
  interchange reader (version, `verified` shape, freshness timestamps, and
  root-index extensions above).
- `strict` adds owners, title, description, status, generation, authority,
  and this repository’s preferred types and tag spelling. It is the right
  profile for Okmate’s own `knowledge/` tree. It is the wrong default for a
  handbook or data catalog that uses other types.

Do not treat a Strict type warning as a malformed bundle. Do not treat a
YAML parse error as a style nit.
