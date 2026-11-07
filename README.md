# vaultrs

A terminal browser for an Obsidian vault. Projects on the left, their notes in the
middle, the note itself on the right — read-only, it never writes to the vault.

**Status:** data layer done and tested. The terminal UI is still the ratatui counter
template, which milestone 1 replaces with the project list. Nothing here is installed
as a release yet.

## What works today

```console
$ cargo run -- --dump
cheatrs
failsafe
ram-watch-rs
```

`--dump` resolves `$HOME/Documents/vault/projetos`, reads its direct children and
prints one project name per line, sorted. The rules are the contract of the loader:

- only directories (files, symlinks to files and hidden names are skipped);
- a child that cannot be read, or whose name is not valid UTF-8, is skipped — one bad
  entry never aborts the load;
- the root failing to open is the only fatal error;
- an empty root is an empty list, not an error;
- ordering is deterministic: sorted by name, not by whatever the filesystem returns.

With `--dump` absent the binary starts the TUI, which for now is the counter template.

## Build and run

```console
cargo run -- --dump   # data layer, no terminal modes involved
cargo run             # TUI: counter template, q quits
cargo test            # 6 tests: 2 unit, 4 integration
```

Rust stable, no toolchain pin. Developed on CachyOS (Arch) with rustc 1.99; nothing in
the code is platform specific beyond `$HOME`.

## Layout

| Path | Role |
|---|---|
| `src/model.rs` | `Project { name, path }` |
| `src/data.rs` | the only module that touches the disk |
| `src/main.rs` | the edge: `$HOME` resolution, `--dump`, terminal loop |
| `tests/load_projects.rs` | integration suite over `tests/fixtures/projetos/` |

`main.rs` is a separate crate root from `lib.rs`: the binary consumes
`vaultrs::data::load_projects`, and the library knows nothing about `$HOME`. That is
what lets the test suite run against a fixture instead of a real vault.

## Roadmap

The target layout, once the list exists:

```
┌ projects ──────┬ notes ───────────────┬ note ──────────────────────────┐
│ cheatrs        │ context.md           │ # cheatrs                      │
│ failsafe       │ plan.md              │ ...                            │
│ ram-watch-rs   │ decisions.md         │ ## 2026-11-04 — ...            │
└────────────────┴──────────────────────┴────────────────────────────────┘
```

- **M1** — navigable project list: `j`/`k`/arrows with clamp, `q` quits, `App` and `ui`
  split out of `main.rs`, selection tested without a terminal.
- **M2** — note list per project, line-based markdown rendering (`#`, code fences,
  `-`, `[[wikilinks]]`), no markdown crate.
- **M3** — full-text search over note bodies, ranking, `Enter` to open.
- **M4** — `Enter` on a wikilink with a back stack, `d` for a timeline of every
  `decisions.md`, installable release with a README demo.

## Why

Reading a note is what Obsidian is for. The point of this one is opening inside the
terminal in the middle of other work, jumping by wikilink and reading the decision
timeline without leaving the keyboard.

## Name

`vaultrs` on crates.io is the HashiCorp Vault client — unrelated. This repository keeps
the name locally; publishing would need a different crate name (`vault-tui` is free).
