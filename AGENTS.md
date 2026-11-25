# lib-url-rfc

`url_rfc` — the URL parser for the `parse` port
(`lib-url-rfc`/`lib-url-whatwg` register themselves into
`lib-parse`'s parser table), plus its `url_rfc.*` ops.

## Layout

| path | role |
| --- | --- |
| `src/index.hnl`, `src/index_flat.hnl` | the .hnl surface |
| `src/parse.hnl` | the parser implementation |
| `src/internal/slist.hnl` | shared internals |
| `src/url_rfc.test.hnl` | the .hnl test suite |
| `pkg.hnl`, `scripts/hnl-gate.sh`, `ci.sh` | manifest / gate / bupha entry |

## Contract

- RFC 3986 URL parsing registered into the `parse` port's
  caller-supplied parser table (Rule B — `lib-parse` knows no
  parsers).
- Note: README mentions a Rust crate (`iri-string` backend) +
  `Cargo.toml`/`xtask`, but the repo currently ships only `.hnl`
  sources — no `Cargo.toml` is present.

## Verification

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
