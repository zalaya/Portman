# Contributing

## Setup

```bash
git clone https://github.com/zalaya/Portman.git
cd Portman
cargo run
```

## Layout

- `src/data/` — talks to the OS: port scan, process lookups.
- `src/app/` — application state, one file per concern.
- `src/terminal/` — `input.rs` (key press → `App` mutation, no I/O) and `event_loop.rs` (the actual terminal loop).
- `src/cli/` — non-interactive mode: `args.rs` (parsing), `report.rs` (what to report), `commands.rs` (`--json`/`check`).
- `src/system/` — thin wrappers around OS facilities: `clipboard.rs`, `browser.rs`.
- `src/ui/` — rendering only: `chrome/`, `panes/`, `overlays/`, `widgets/`, `theme.rs`.
- `tests/` — integration tests, fixtures in `tests/support/`.
- `benches/` — `criterion` benchmark for the search/sort hot path.

## Before opening a PR

```bash
cargo build
cargo clippy --all-targets -- -D warnings
cargo test
```

## Style

- No comments — name things so they don't need one.
- Keep changes scoped; no drive-by refactors.
- Adding a keybinding? Update `src/ui/overlays/help.rs`.
- Pure logic gets a unit test next to it; anything crossing `app`/`input` goes in `tests/`.

## Bugs and features

Open an issue. Security issues go through [SECURITY.md](SECURITY.md) instead.
