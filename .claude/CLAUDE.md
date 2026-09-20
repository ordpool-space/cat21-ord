# CLAUDE.md

Fork of the [ord client](https://github.com/ordinals/ord) that indexes CAT-21 via the **fake inscription** approach, on branch `index-cat21`. Family-wide rules live in the workspace `CLAUDE.md`; this file is fork-specific.

ord is taught to treat `nLockTime=21` transactions as inscriptions. They are not real ones (no `OP_FALSE OP_IF` envelope exists on chain), but ord does not need to know: once they count as inscriptions, sat tracking, transfers and the whole API work unchanged. Approach borrowed from `labitbu/pathologies`.

## RULE: Lint before every commit

```bash
cargo fmt -- --check
RUSTFLAGS="--deny warnings" cargo clippy --all --all-targets
cargo test
cargo build --release
```

Why: CI runs both checks with `--deny warnings` and fails on any warning or formatting issue.

## RULE: Keep useful comments

- Trim the text inside a comment (no bombast, no LLM-speak, no before/after history); keep the block.
- Specifically keep the CAT-21 indexing rationale and the "this differs from upstream ord because..." notes around `// CAT-21 😺 - START` blocks.

Why: the next upstream merge needs to know why a line diverges, and that cannot be reconstructed from the code.
Ref: workspace `CLAUDE.md` "Keep useful comments (JSDoc AND inline 'why')".

## RULE: Every CAT-21 change is marked and minimal

- Wrap all CAT-21 code in `// CAT-21 😺 - START` and `// CAT-21 😺 - END`.
- Add new code alongside existing code. Do not change existing lines where it can be avoided.
- Touch only what is necessary.

Why: every changed line is a potential conflict on the next rebase onto upstream.

## RULE: Keep templates upstream-clean, put display changes in the middleware

- Never add `%% if index_cat21` conditionals to templates for display-only changes; they belong in `cat21_text_layer`.
- The one exception is `inscription.html`'s traits section, which needs `txid`, `block_hash`, `fee` and `weight`, data only the template can reach.

## RULE: Fork management, squash and rebase only

- **PR #1 stays open permanently.** It documents the full delta against upstream. Never merge it.
- Squash all changes into a single commit on `index-cat21`. One commit = one diff = an easy rebase.
- When upstream releases, REBASE `index-cat21` onto `upstream/master` and force push. Never merge; merge commits create clutter.

## RULE: Run the server from systemd, never by hand

- Use the unit files in `deploy-ord-dev/`: `ord.service` (`CAP_NET_BIND_SERVICE` for port 80, auto-restart, journald) and `bitcoind.service` (auto-restart).
- Never start either with `nohup`.

## RULE: Update the docs that describe the fork when the fork changes

- Review `FORK.md`, `.claude/CLAUDE.md`, `README.md` and the [wiki](https://github.com/ordpool-space/cat21-ord/wiki) for anything the change made stale.
- Wiki clone: `git@github-ord-dev:ordpool-space/cat21-ord.wiki.git`.

## Flags

| flag | status |
|---|---|
| `--index-cat21` | REQUIRED, enables CAT-21 indexing mode |
| `--index-sats` | REQUIRED, sat tracking for ordinal theory |
| `--index-addresses` | recommended, address lookups |
| `--no-index-inscriptions` | INCOMPATIBLE: cats ARE inscriptions here |

Without `--index-cat21` ord behaves 100% like upstream. With it, the indexer ignores all real inscriptions (it does not even parse tapscript witnesses), checks `nLockTime == 21` on every transaction, creates a fake empty envelope for matches, and uses `first_cat21_height` (block 815855, the genesis cat) instead of `first_inscription_height`.

## Key files

| file | CAT-21 change |
|---|---|
| `src/options.rs` | `--index-cat21` CLI flag |
| `src/settings.rs` | wires the flag through; overrides `first_inscription_height()` when active |
| `src/chain.rs` | `first_cat21_height()`, block 815855 |
| `src/index/updater/inscription_updater.rs` | the nLockTime check and fake envelope creation |
| `src/subcommand/server.rs` | `cat21_text_layer` middleware, `/cat/` and `/cats` routes |

## Display layer

<!-- long-rule: two middlewares with ordering constraints and a corruption guard -->

Two axum middlewares in `server.rs` hold every cat21 display transformation.

**`cat21_url_rewrite` (inbound)** rewrites `/cat/` to `/inscription/` and `/cats` to `/inscriptions` BEFORE route matching. It wraps the main router from outside via `Router::new().fallback_service(inner).layer(...)`, because `Router::layer()` runs after route matching and would be too late.

**`cat21_text_layer` (outbound)** transforms HTML, CSS and JSON bodies when `--index-cat21` is active:

- Terminology: `Inscription` to `Cat`, `inscription` to `cat`, across HTML text, CSS selectors (`.inscription` to `.cat`) and JSON field names (`"inscriptions"` to `"cats"`).
- **Sat-name protection**: `protect_field` prevents data corruption for sat names containing "inscription" (names are base-26 numbers). Before the blanket replace it scans `"name":"..."` JSON, `<dt>name</dt><dd>...</dd>` HTML and `/sat/...` URLs, swaps in a placeholder, and restores afterwards. Handles multiple occurrences.
- Home title `<title>Ordinals</title>` to `<title>CAT-21</title>`; nav `<sup>beta</sup>` to `<sup>CAT-21</sup>` plus the genesis cat logo link.
- Injects `cat21-page.css` and the `public-pixel.woff2` preload after `modern-normalize.css`.
- Strips `<h2>0 Runes</h2>` (always 0 in cat21 mode). Transaction page gains a line break after "Transaction", the txid, and an ordpool.space link.
- `Content-Length` is recalculated after replacement.

**Layer ordering**: list the outbound `cat21_text_layer` BEFORE the `Extension` layers (innermost in the onion) so it can extract `ServerConfig`, but AFTER `CompressionLayer` so it sees uncompressed bodies.

## Wallet in cat mode (`ord --index-cat21 wallet ...`)

`cat21_text_layer` rewrites JSON too, which breaks ord's own wallet: it is a serde consumer of the canonical ord schema, so a renamed `/status` (`blessed_inscriptions` to `blessed_cats`) fails to deserialise, and every wallet command fetches `/status` in `WalletConstructor::build`.

The fix is symmetric with the server. `WalletConstructor`/`Wallet` read `settings.index_cat21()` and run every response body through `cat21_decat_json` (`src/wallet.rs`, the inverse rename) before parsing, so the canonical `api::*` structs deserialise unchanged. Without the flag the wallet is vanilla ord and decat is a no-op.

Caveat: the reverse pass is a blanket substring replace. The forward pass is safe on bech32 because "inscription" contains no `i`/`o`; the reverse is not, because "cat" is valid bech32, so a rare address containing the literal "cat" would be rewritten. The field names the wallet relies on reverse cleanly. See the `cat21_decat_json` doc comment.

## What comes free, and what needs no routes

Once cats appear as inscriptions, ord handles sat assignment (first sat of the first output), transfer tracking, API endpoints (`/inscription/<txid>i0` and `/cat/<txid>i0`), address lookups and database storage.

No extra routes exist: the inbound rewrite maps `/cat/{id}` and `/cats` onto the existing `/inscription/` and `/inscriptions` routes, so the route table is zero-diff with upstream.

## Documentation

`FORK.md` describes what the fork changes and how. The [wiki](https://github.com/ordpool-space/cat21-ord/wiki) holds the human setup guides (Developer HowTo, Bitcoin guides, Ord guide). Full setup for the commands above is in the wiki.
