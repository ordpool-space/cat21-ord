# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Fork of the [ord client](https://github.com/ordinals/ord) with CAT-21 indexing via the **fake inscription** approach. Branch: `index-cat21`.

The idea: teach ord to recognize `nLockTime=21` transactions as inscriptions. They're not real inscriptions — there's no `OP_FALSE OP_IF` envelope on-chain — but ord doesn't need to know that. Once it treats them as inscriptions, all of ord's infrastructure (sat tracking, transfers, API) works automatically.

Inspired by `labitbu/pathologies` (Labitbu's ord fork for indexing pathologies).

## Build, Lint & Test

```bash
cargo build --release
cargo fmt -- --check                              # format lint
RUSTFLAGS="--deny warnings" cargo clippy --all --all-targets  # clippy lint
cargo test
# See the wiki for full setup: https://github.com/ordpool-space/cat21-ord/wiki
```

### HARD RULE: Always lint before committing
Run both `cargo fmt -- --check` AND `RUSTFLAGS="--deny warnings" cargo clippy --all --all-targets` before every commit. CI runs both checks with `--deny warnings` and will fail the build on any warning or formatting issue.

## CAT-21 Development Rules

### Code Organization
- Mark all CAT-21 code blocks with `// CAT-21 😺 - START` and `// CAT-21 😺 - END`
- Minimize changes to core ord code — the fork must stay easily mergeable with upstream
- Don't change existing lines if possible; prefer adding new code alongside existing code

### The `--index-cat21` Flag
Without the flag, ord behaves 100% like upstream. With `--index-cat21`, the indexer:
- Ignores all real inscriptions (doesn't even parse tapscript witnesses)
- Checks `nLockTime == 21` on every transaction
- Creates a fake empty envelope for matching transactions
- Uses `first_cat21_height` (block 815855) instead of `first_inscription_height`

### Key Files (CAT-21 changes)
1. **`src/options.rs`** — `--index-cat21` CLI flag definition
2. **`src/settings.rs`** — Wires flag through settings, overrides `first_inscription_height()` when active
3. **`src/chain.rs`** — Defines `first_cat21_height()` (block 815855, genesis cat)
4. **`src/index/updater/inscription_updater.rs`** — Core logic: nLockTime check + fake envelope creation
5. **`src/subcommand/server.rs`** — `cat21_text_layer` middleware + `/cat/` and `/cats` routes

### Display Layer: `cat21_text_layer` Middleware

All cat21 display transformations are centralized in two axum middlewares in `server.rs`:

**`cat21_url_rewrite` (inbound)** — Rewrites `/cat/` → `/inscription/` and `/cats` → `/inscriptions` URLs before route matching. Applied via an outer `Router` wrapping the main one with `fallback_service`, because `Router::layer()` runs AFTER route matching and can't rewrite URLs in time.

**`cat21_text_layer` (outbound)** — Transforms HTML, CSS, and JSON response bodies when `--index-cat21` is active:
- **Terminology**: `Inscription` → `Cat`, `inscription` → `cat` (applies to HTML text, CSS class selectors like `.inscription` → `.cat`, and JSON field names like `"inscriptions"` → `"cats"`)
- **Sat name protection**: The `protect_field` helper prevents data corruption for sat names that contain "inscription" (sat names are base-26 encoded numbers). Before the blanket replacement, it scans for "inscription" inside `"name":"..."` JSON fields, `<dt>name</dt><dd>...</dd>` HTML, and `/sat/...` URLs, temporarily replacing with a placeholder that's restored afterward. Handles multiple occurrences.
- **Home title**: `<title>Ordinals</title>` → `<title>CAT-21</title>`
- **Nav**: Replaces `<sup>beta</sup>` with `<sup>CAT-21</sup>` and injects the genesis cat logo link
- **CSS/font**: Injects `cat21-page.css` stylesheet and `public-pixel.woff2` font preload after `modern-normalize.css`
- **Runes**: Strips `<h2>0 Runes</h2>` (always 0 in cat21 mode)
- **Transaction page**: Adds line break after "Transaction", shows txid, adds ordpool.space link
- **Content-Length**: Recalculated after body replacement to match the new body size

**Design principle**: Keep templates upstream-clean. Never add `%% if index_cat21` conditionals for display-only changes — put them in the middleware instead. The only exception is `inscription.html`'s traits section, which needs dynamic data attributes (`txid`, `block_hash`, `fee`, `weight`) that only the template has access to.

**Layer ordering**: The outbound `cat21_text_layer` must be listed before the `Extension` layers (innermost in the onion) so it can extract `ServerConfig`, but after `CompressionLayer` (so it processes uncompressed bodies). The inbound `cat21_url_rewrite` wraps the entire router from outside via `Router::new().fallback_service(inner).layer(...)`.

### Routes

No extra routes are needed. The inbound URL rewrite ensures `/cat/{id}` and `/cats` URLs map to the existing `/inscription/` and `/inscriptions` routes. The original routes are untouched — zero diff with upstream.

### What ord handles automatically
Once cats appear as inscriptions:
- Sat assignment (first sat of first output)
- Transfer tracking across transactions
- API endpoints (`/inscription/<txid>i0` and `/cat/<txid>i0`)
- Address lookups
- Database storage

### Important flags
- `--index-cat21` — REQUIRED (enables CAT-21 indexing mode)
- `--index-sats` — REQUIRED (sat tracking for ordinal theory)
- `--index-addresses` — recommended (address lookups)
- `--no-index-inscriptions` — INCOMPATIBLE with `--index-cat21` (cats ARE inscriptions)
- Do NOT use `--no-index-inscriptions` — cats ARE inscriptions in this approach

### Fork Management
- **PR #1 stays open permanently** — it documents the full delta against upstream. Never merge it.
- **Squash all changes into a single commit** on `index-cat21`. One commit = one diff = easy rebase. No commit clutter.
- **Rebase onto upstream, never merge** — when upstream releases a new version, rebase `index-cat21` onto `upstream/master` and force push. Merge commits create clutter.
- **Keep changes minimal** — every line we change is a potential merge conflict. Only touch what's necessary.

### Server Deployment

Use the systemd service files in `deploy-ord-dev/` to run services in production:
- `ord.service` — runs ord with `CAP_NET_BIND_SERVICE` (port 80), auto-restart, journald logging
- `bitcoind.service` — runs bitcoind with auto-restart

**Never start these manually with `nohup`.**

### Documentation
- `FORK.md` — technical description of what this fork changes and how
- [Wiki](https://github.com/ordpool-space/cat21-ord/wiki) — setup guides for humans (Developer HowTo, Bitcoin guides, Ord guide)
- When making changes, review `FORK.md`, `.claude/CLAUDE.md`, `README.md`, and the [wiki](https://github.com/ordpool-space/cat21-ord/wiki) for outdated information and update them. The wiki repo can be cloned at `git@github-ord-dev:ordpool-space/cat21-ord.wiki.git`.
