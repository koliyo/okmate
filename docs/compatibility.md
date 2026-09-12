# OKF reader compatibility inventory

This inventory documents current `okf` / `okmate` diagnostics after format,
evidence, and local style were split. Executable fixtures live in
[`okf/tests/fixtures/compatibility/`](../okf/tests/fixtures/compatibility/).
They are small synthetic trees. Third-party repositories are not golden
fixtures.

Rule ownership:

- **OKF format** — portable parse and citation rules the engine enforces
  for any producer. These stay `Diagnostic.layer = format`.
- **Okmate Evidence** — optional owners-and-evidence fields without a
  product type list. `okmate check --profile evidence`. Layer `evidence`.
- **Okmate Strict** — Evidence plus this product’s type names, `status`,
  and `domain/` tag spelling. Default for `check` / `view`. Still available
  as `--profile strict`.
- **Okmate style** — optional `<bundle>/okmate.toml` advice in the
  application, not the engine. Layer `style`. Warnings only; they do not
  fail `check` or enter `has_errors()` / the governance diagnostic count.
- **Okmate isolation** — keep record bodies inert and unsafe HTML out of
  rendered output. HTML comments are stripped from HTML and are not
  authoring errors.

JSON compatibility: format findings omit `layer` (the default). Evidence
and style findings include `"layer": "evidence"` or `"layer": "style"`.
Consumers that ignore unknown fields keep working. Terminal lines insert
` evidence` or ` style` after the severity for those layers.

| Fixture | Owner | Code | Level | Profile | What you will see |
| --- | --- | --- | --- | --- | --- |
| `malformed-yaml` | OKF format (`okf` parse) | `OKF1003` | error | all | Broken YAML is a parse failure. The Markdown body may still be captured. |
| `malformed-root-index` | OKF format (`okf` parse) | `OKF1003` | error | all | A reserved root index with broken YAML is still a parse failure. |
| `citation-missing-definition` | OKF format (citations) | `OKF4003` | error | all | A `[^id]` footnote with no definition in the body. |
| `citation-unused-source` | OKF format (citations) | `OKF4002` | warning | all | A `sources[].id` never used by a body footnote. |
| `peer-root-extension` | OKF format (`okf` load) | `OKF1011` | warning | all | Extra root `index.md` keys are ignored; only `okf_version` is applied. This is advisory, not a prohibition of peer extensions. |
| `advisory-root-and-malformed` | OKF format | `OKF1011` + `OKF1003` | warning + error | all | An advisory root extension does not hide malformed concept YAML. |
| `v0.1-version` | OKF format | — | clean | all | String `okf_version` `0.1` and `0.2` are accepted. Other values remain `OKF1012`. |
| `verified-mapping` | OKF format | — | clean | all | A single `verified` mapping is normalized to a one-element list. |
| `stale-after-timestamp` | OKF format | — | clean | all | `stale_after` accepts `YYYY-MM-DD` or RFC 3339. Freshness compares the civil date. |
| `unknown-metadata` | OKF format (optional metadata) | `OKF2001` | warning | evidence and strict | Unknown fields are **preserved**. `base` does not warn; evidence/strict do. |
| `type-only-workflow` | Okmate Evidence | `OKF2003` | error | evidence and strict | A custom `type` is readable on `base`. Evidence requires title, description, generated, authority, and owners. |
| `unknown-type` | Okmate Strict | `OKF2002` | warning | strict only | `type: Workflow` is valid on `base` and `evidence` and warns on `strict`. |
| `evidenced-runbook` | Okmate Strict | `OKF2002` / `OKF2004` / `OKF2005` | warning / error | strict only | Evidence accepts arbitrary types and tags. Strict still wants product types and `domain/` prefixes. |
| `missing-domain-tag` | Okmate Strict | `OKF2004` | error | strict only | Strict requires at least one `domain/*` tag and only `domain/`, `integration/`, `concern/`, `audience/` prefixes (`OKF2005`). |
| `html-comment` | Okmate isolation | — | clean | all | HTML comments are not authoring errors. They are stripped from `article_html`. |
| `unsafe-html` | Okmate isolation | `OKF2009` | error | all | Active markup is forbidden and is not copied into rendered output. |
| `rocdown-declaration` | Okmate isolation | `OKF2007` | error | all | `@page` / `@render` and other Rocdown declarations are forbidden in knowledge bodies. |

Local style (`OKMATE5001` undeclared type, `OKMATE5002` type path,
`OKMATE5003` other convention advice) is produced by `okmate` when
`<bundle>/okmate.toml` exists. Absence means no vocabulary noise. The
portable `okf` crate never emits those codes.

## How to read a check report

- **Format and evidence errors** block a clean `check` and `build`.
- **Warnings** (including style) do not fail `check`.
- `base` is the portable reader: v0.1/v0.2, mapping `verified`, timestamp
  freshness, HTML comments, and advisory root-index extensions.
- `evidence` adds owners, title, description, generation, and authority
  without product type names or `domain/` tags.
- `strict` is the right profile for Okmate’s own `knowledge/` tree. It is
  the wrong default for a handbook or data catalog that uses other types.
  `check` and `view` still default to `strict`; new `okmate init --agents`
  instructions select `--profile evidence` explicitly.

Do not treat a Strict type warning as a malformed bundle. Do not treat a
YAML parse error as a style nit. Do not treat a style warning as a format
error.
