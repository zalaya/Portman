# Portman

A fast, keyboard-driven TUI for seeing what's listening on your machine's ports — and shutting it down.

[![CI](https://github.com/zalaya/Portman/actions/workflows/ci.yml/badge.svg)](https://github.com/zalaya/Portman/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Every port is scored by how reachable it is — 🟢 Safe (loopback only), 🟡 Watch (exposed), 🔴 Critical (exposed *and* a typically sensitive service like a database or remote admin) — so the risky ones stand out instead of scrolling past.

## Install

macOS or Linux:

```bash
curl -fsSL https://raw.githubusercontent.com/zalaya/Portman/main/install.sh | sh
```

Windows (PowerShell):

```powershell
irm https://raw.githubusercontent.com/zalaya/Portman/main/install.ps1 | iex
```

No Rust toolchain needed either way. Want a specific version, or to build from source instead?

```bash
git clone https://github.com/zalaya/Portman.git
cd Portman
cargo install --path .
```

Run it with `portman` — press `Ctrl+K` for keybindings.

## Development

```bash
cargo test
cargo bench
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for the project layout, and [SECURITY.md](SECURITY.md) to report a vulnerability.

## License

MIT — see [LICENSE](LICENSE).
