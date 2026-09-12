# Worked OKF examples

Each directory is a complete bundle. Run the documented `okmate check`
command; the expected result is part of the example.

| Example | Purpose | Start navigation | Profile | Expected check |
| --- | --- | --- | --- | --- |
| [minimal](minimal/) | Small standalone corpus with no taxonomy | Root index → scope | `base` | 0 errors, 0 warnings |
| [software-archive](software-archive/) | Type-first product work archive | Architecture / Decisions indexes | `strict` | 0 errors, 0 warnings |
| [engineering-handbook](engineering-handbook/) | Topic and question-oriented practice library | Frozen root questions → explanations, guides, dated research, audits | `base` | 0 errors, 0 warnings |
| [operations](operations/) | Service map and runbooks | Root → services / runbooks | `base` | 0 errors, 0 warnings |
| [data-catalog](data-catalog/) | Datasets, tables, and a metric | Root → datasets / tables / metrics | `base` | 0 errors, 0 warnings |

```sh
okmate check docs/examples/minimal --profile base
okmate check docs/examples/software-archive --profile strict
okmate check docs/examples/engineering-handbook --profile base
okmate benchmark docs/examples/engineering-handbook/retrieval.toml \
  docs/examples/engineering-handbook --profile base
okmate benchmark docs/examples/software-archive/retrieval.toml \
  docs/examples/software-archive --profile strict
okmate check docs/examples/operations --profile base
okmate check docs/examples/data-catalog --profile base
```

`strict` on the handbook, operations, or data examples currently warns
(`OKF2002`) or errors (`OKF2004`) because those types and tags are outside
Okmate’s product vocabulary. That is expected; see
[compatibility.md](../compatibility.md). Use `--profile evidence` when the
records already carry owners and generation and you want those checks
without the product type list.
