# Portman

A fast, keyboard-driven TUI for seeing what's listening on your machine's ports — and shutting it down.

[![CI](https://github.com/zalaya/Portman/actions/workflows/ci.yml/badge.svg)](https://github.com/zalaya/Portman/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Every port is scored by how reachable it is — 🟢 Safe (loopback only), 🟡 Watch (exposed), 🔴 Critical (exposed *and* a typically sensitive service like a database or remote admin) — so the risky ones stand out instead of scrolling past.

## Install

Download a binary from the [Releases page](https://github.com/zalaya/Portman/releases), or build from source:

```bash
git clone https://github.com/zalaya/Portman.git
cd Portman
cargo install --path .
```

## Usage

```bash
portman
```

| Key         | Action                                          |
| ----------- | ------------------------------------------------ |
| `↑` / `↓`   | Move selection                                   |
| type        | Search by port, bind, process or PID             |
| `Tab`       | Cycle sort key (port, bind, process, PID, risk)  |
| `Enter`     | Open the actions menu (kill, copy PID/address)   |
| `Delete`    | Kill the selected process                        |
| `Ctrl+T`    | Cycle the color theme                            |
| `Ctrl+L`    | Toggle the activity log                          |
| `Ctrl+R`    | Refresh now                                      |
| `Ctrl+K`    | Toggle the keybindings help                      |
| `Esc`       | Quit, or close whatever's open                    |

Killing a process you don't own usually needs `sudo`.

## Development

```bash
cargo test
cargo bench
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for the project layout, and [SECURITY.md](SECURITY.md) to report a vulnerability.

## License

MIT — see [LICENSE](LICENSE).
