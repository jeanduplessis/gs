# gs

`gs` is a Rust read-only CLI that prints an Enhanced status view for the current Git repository.

It uses a Git library backend (`git2`/libgit2), not the installed `git` command, for repository inspection. The binary is intentionally thin: it parses CLI options, writes stdout/stderr, and returns exit codes; reusable library modules provide the Repository inspector, Change model, Diff/stat calculator, and Renderer.

## Usage

```sh
gs [--color=auto|always|never]
```

`--color` defaults to `auto`:

- `auto`: emit ANSI color only when stdout is a TTY.
- `always`: emit deterministic ANSI 256-color styling.
- `never`: emit plain output.

Outside a Git repository, `gs` prints this to stderr and exits `1`:

```text
gs: not a git repository
```

## Output contract

The Enhanced status view starts with a Branch header, followed by visible change Sections. Empty Sections are hidden.

A clean repository renders branch context plus Clean repository output:

```text
main
✓ working tree clean
```

A repository with changes renders Sections in this order:

1. `Staged`
2. `Tracked`
3. `Untracked`

Section headers include Section count, i.e. rendered Entry count:

```text
main ↑1 ↓2
Staged (1)
  M src/lib.rs  +3/-1

Tracked (1)
  D old.txt     +0/-4

Untracked (1)
  ? notes.txt   +2/-0
```

Each Entry line uses:

- two-space indentation
- one Git-letter status symbol: `M`, `A`, `D`, `R`, or `?`
- Repository-root-relative Display path
- padding to align the Entry stats column and vertically align the `/` separator across Entries
- Entry stats as `+N/-N` for Known text stats or `+?/-?` for Unknown stats

## Behavior details

- `Staged` contains index changes that are commit-ready.
- `Tracked` contains unstaged worktree changes to tracked files.
- `Untracked` contains untracked, non-ignored files.
- Ignored files are excluded; there is no ignored-files Section.
- Entries sort alphabetically by Display path.
- Paths are displayed relative to the repository root, independent of invocation directory.
- Partially staged files render twice: once in `Staged` and once in `Tracked`, with separate Entry stats.
- Rename display uses `old/path -> new/path`; renames sort by destination path.
- Untracked text files render all lines as additions: `+N/-0`.
- Binary, non-line-oriented, and parent-visible submodule path-level changes render Unknown stats: `+?/-?`.
- Submodule internals are not inspected.
- Branch headers render `branch ↑ahead ↓behind`; missing or zero upstream divergence values are omitted.
- Detached HEAD renders `detached @ <short-sha>`.

## Color contract

Forced color output uses deterministic ANSI 256-color styling:

- additions: green (`38;5;2`)
- deletions: red (`38;5;1`)
- stats separator `/`: muted gray (`38;5;244`)
- Staged Section and Entries: green (`38;5;2`)
- Tracked Section and Entries: tan (`38;5;180`)
- Untracked Section and Entries: muted gray (`38;5;245`)

## Development and testing

Run:

```sh
cargo test
```

Tests verify observable behavior through public interfaces:

- Temporary Git repository tests cover clean repositories, outside-repository errors, untracked files, Ignored file exclusion, tracked and staged changes, Partially staged files, Rename display, Unknown stats, Branch header divergence, Detached HEAD, and submodule path-level behavior.
- Renderer tests cover plain output, forced-color output, `auto` color behavior, Section count, hidden empty Sections, aligned stats columns, Clean repository output, and Unknown stats.
