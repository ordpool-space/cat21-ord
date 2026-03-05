
# CAT-21 Development Summary

## The Fake Inscription Approach

CAT-21 cats are not real inscriptions — there's no `OP_FALSE OP_IF` envelope in the transaction. But ord doesn't need to know that. By creating a fake inscription envelope for every `nLockTime=21` transaction, ord's entire inscription pipeline kicks in automatically: sat tracking, transfer detection, API endpoints, address lookups.

## How It Works — The `--index-cat21` Flag

Without the flag, ord behaves 100% like upstream. All existing tests pass. With `--index-cat21`, the indexer switches to CAT-21 mode: real inscriptions are completely ignored (not even parsed), and only `nLockTime=21` transactions are indexed.

### 1. CLI flag — `src/options.rs` + `src/settings.rs`

The `--index-cat21` flag (or `ORD_INDEX_CAT21` env var) activates CAT-21 indexing. When active, `first_inscription_height()` returns `first_cat21_height` (block 815855) instead of the normal inscription start height.

### 2. Core logic — `src/index/updater/inscription_updater.rs`

When `--index-cat21` is active, `from_transaction()` is bypassed entirely. Instead, the updater checks `nLockTime == 21` directly and creates a fake empty envelope. Real inscriptions are completely ignored — not even parsed.

### 3. Start height — `src/chain.rs`

`first_cat21_height()` defines block 815855 — the genesis cat block (first `nLockTime=21` transaction in Bitcoin history). This prevents ord from scanning earlier blocks.

## What Ord Gives Us For Free

- **Sat assignment**: first sat of first output (exactly what CAT-21 needs)
- **Transfer tracking**: every time the sat moves, ord tracks it
- **API**: `/inscription/<txid>i0` gives current owner, sat number, transfer history
- **Address lookups**: which cats does this address own?
- **No custom database tables**: ord's existing redb handles everything

## Cat Numbers = Inscription Numbers

Because `--index-cat21` ignores all real inscriptions, the only things in the index are cats. This means ord's inscription number IS the cat number. The genesis cat (block 815855) gets inscription number 0 = **cat #0**. The next cat gets inscription number 1 = **cat #1**, and so on. This matches the official "cat number" as shown on the [Dune dashboard](https://dune.com/ethspresso/cat21).

No translation between inscription numbers and cat numbers is needed — they are the same thing in this dedicated index.

## Server: Display Layer + Routes — `src/subcommand/server.rs`

### `cat21_text_layer` Middleware

All display transformations are centralized in one axum middleware. Templates render plain upstream content; the middleware transforms HTML and CSS responses:

- `Inscription` → `Cat` / `inscription` → `cat` (text + CSS selectors)
- Home page title → `CAT-21`
- Nav superscript → `CAT-21` + genesis cat logo link
- Injects `cat21-page.css` + `public-pixel.woff2` font
- Strips `0 Runes` heading

### Cat Routes

`/cat/{id}` and `/cats` routes are added alongside the original `/inscription/` and `/inscriptions` routes. Both work — the original routes are untouched for zero diff with upstream.

### Cat Preview + Traits

- **Preview**: `PreviewCat21Html` renders the cat SVG client-side via `cat21.js`, using `SHA256(txId + blockHash)` and fee rate
- **Traits**: `inscription.html` includes a `#cat21-traits` div with data attributes; `cat21-traits.js` computes and renders traits via DOM API

### Static Assets

- `static/cat21-page.css` — orange theme, Public Pixel font, nav styling
- `static/cat21-traits.css` + `cat21-traits.js` — trait display on inscription page
- `static/cat21.js` — cat SVG generation (from ordpool-parser)
- `static/cat21-logo.svg`, `static/ordpool-logo.png` — nav icons
- `static/preview-cat21.css` — orange background for cat previews

## Inspiration

Discovered from the Labitbu project (`labitbu/pathologies`), which uses the same fake inscription trick to index WebP images embedded in Taproot control blocks. Their detection parses 4129-byte witness items for a NUMS key. CAT-21 detection checks `nLockTime == 21`.
