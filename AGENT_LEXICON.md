# Agent Lexicon

## Canonical Terms

| Term | Agent meaning | Use this when | Avoid |
|---|---|---|---|
| **`gs`** | The Rust CLI binary/package that prints the enhanced Git status view. | Naming the command, package, executable, docs, tasks, and user-facing CLI behavior. | `git-smart-status`, `git smart-status`, smart status CLI, custom status CLI |
| **Enhanced status view** | The read-only output produced by `gs`: branch header plus grouped file-change sections with per-entry stats. | Referring to the product output as a whole. | git status replacement, richer status, smart status, dashboard |
| **Read-only CLI** | A CLI that inspects repository state and never stages, unstages, discards, commits, or prompts for interactive actions. | Defining scope, safety, UX, and tests. | interactive CLI, Git workflow tool, staging tool |
| **Git library backend** | Repository data collection through Rust Git/libgit2 bindings, not subprocess calls to the installed `git` binary. | Describing implementation constraints for status/diff/branch data. | shelling out, Git CLI backend, `git status` parser |
| **Library-plus-thin-binary** | Architecture where reusable core modules implement behavior and the binary only handles CLI parsing, IO, and exits. | Structuring implementation tasks and tests. | single binary crate, workspace split, monolith |
| **Repository inspector** | Module responsibility that opens the repo and reads branch, upstream, status, ignore-aware changes, and parent-level submodule changes. | Naming repository state collection responsibilities. | Git parser, status reader, repo scanner |
| **Change model** | Normalized domain representation of branch state, sections, entries, symbols, paths, and stats. | Passing repository data to rendering/tests. | raw status, output rows, view model unless specifically rendering-only |
| **Diff/stat calculator** | Module responsibility that computes known `+N/-N` stats or unknown `+?/-?` stats for staged, tracked, and untracked entries. | Naming line-stat behavior. | line counter, diff parser, numstat clone |
| **Renderer** | Module responsibility that converts the change model into plain or colored terminal text. | Naming layout, color, alignment, and snapshot behavior. | printer, formatter, UI unless terminal UI is introduced |
| **Branch header** | First output line showing `branch ↑ahead ↓behind`, omitting missing/zero counts, or `detached @ <short-sha>`. | Referring to repository context above sections. | branch summary, status header, title line |
| **Upstream divergence** | Nonzero ahead/behind counts between the current branch and its configured upstream. | Describing `↑N` and `↓N` in the branch header. | remote counts, sync status, tracking summary |
| **Detached HEAD** | Repository state rendered in the branch header as `detached @ <short-sha>`. | Handling non-branch HEAD output. | detached branch, anonymous branch |
| **Section** | One visible grouped change category: `Staged`, `Tracked`, or `Untracked`. | Discussing grouping, counts, colors, ordering, and visibility. | group, bucket, category |
| **Staged section** | Section containing index changes that are commit-ready. | Referring to staged/index entries and green section styling. | cached section, index section, staged files when meaning entries |
| **Tracked section** | Section containing unstaged worktree changes to tracked files. | Referring only to unstaged tracked-file changes and tan section styling. | modified section, changed section, unstaged section |
| **Untracked section** | Section containing untracked, non-ignored files. | Referring to new untracked files and muted gray section styling. | new files section, unknown files, unversioned section |
| **Entry** | One rendered file-change row inside a section. | Discussing status symbol, display path, stats, sorting, and section counts. | file unless the file itself is meant, row unless rendering-only |
| **Section count** | Number of entries in a section shown in the section header, e.g. `Staged (2)`. | Defining section header content. | line total, diff total, file count when entries can duplicate a partially staged path |
| **Git-letter status symbol** | One of `M`, `A`, `D`, `R`, or `?` rendered before an entry path. | Referring to compact status markers. | icons, badges, status words, glyphs |
| **Repository-root-relative path** | File path displayed and sorted relative to the repository root regardless of invocation directory. | Defining path display and sort behavior. | current-directory-relative path, absolute path, Git-relative path |
| **Display path** | The path string shown for an entry; for renames, `old/path -> new/path`. | Discussing sorting, alignment, and rendering. | filepath when rename representation matters |
| **Rename display** | The `old/path -> new/path` display path for renamed entries, sorted by destination path. | Handling renamed files in output and tests. | destination-only path, renamed badge |
| **Partially staged file** | A path with both staged index changes and unstaged tracked worktree changes. | Requiring duplicate entries across `Staged` and `Tracked` with separate stats. | mixed file, partially indexed file, combined staged file |
| **Entry stats** | Per-entry changed-line indicator rendered as known `+N/-N` or unknown `+?/-?`, with `/` separators vertically aligned across entries. | Referring to stats shown at the aligned end of each entry row. | line count, diff count, numstat unless backend-specific |
| **Known text stats** | Entry stats with concrete additions/deletions for text changes, rendered `+N/-N`. | Describing text diff output. | changed lines when deletion/addition split matters |
| **Unknown stats** | Entry stats for binary or otherwise unknown line changes, rendered `+?/-?`. | Handling binary, submodule, or non-line-oriented changes. | binary, no stats, `+0/-0` |
| **Clean repository output** | Output with branch header followed by `✓ working tree clean` when no sections contain entries. | Defining no-change behavior. | no output, clean message only |
| **Color mode** | CLI option `--color=auto|always|never`, defaulting to `auto`. | Controlling ANSI output. | theme, palette option, color flag unless discussing parsing |
| **Deterministic ANSI 256-color styling** | Fixed terminal colors: additions green, deletions red, stats separator muted gray, staged green, tracked tan, untracked muted gray. | Defining color contract and color snapshot expectations. | truecolor, basic ANSI only, custom theme |
| **Ignored file exclusion** | Rule that ignored files never appear in first-version output. | Handling `.gitignore`/ignore-rule behavior. | ignored section, show ignored, muted ignored files |
| **Submodule path-level change** | A parent-repository-visible submodule path change without inspecting inside the submodule. | Handling submodules. | recursive submodule status, submodule internals |
| **Temporary Git repository test** | Integration test that creates an isolated Git repository and asserts observable `gs` behavior. | Testing repository behavior. | mocked Git test, unit-only status test |
| **Rendered output snapshot** | Assertion over user-visible terminal text, including layout and optionally forced color. | Testing renderer contracts. | implementation snapshot, internal model snapshot |
| **PRD epic** | Beads epic task containing the approved product requirements for `gs`; current epic ID is `gs-y9o`. | Creating follow-up implementation tasks. | GitHub issue, task spec, parent story |

## Agent Rules

- Use **`gs`** for the CLI name; do not introduce alternate command names unless updating this lexicon first.
- Use **Enhanced status view** for the whole product output; do not call it a dashboard or interactive tool.
- Use **Read-only CLI** only when no Git-mutating action is available or planned in scope.
- Use **Git library backend** for implementation plans; do not shell out to `git` for status or diff data.
- Use **Tracked section** only for unstaged worktree changes to tracked files; do not use it for all Git-tracked files.
- Use **Entry** for a rendered section row; use file/path only when not discussing duplicated partially staged entries.
- Use **Section count** for entry counts only; do not aggregate line totals in section headers.
- Use **Repository-root-relative path** for path display and sorting; do not vary output by current working directory.
- Use **Display path** when rename formatting can change the path string.
- Render **Partially staged file** as two entries: one in **Staged section**, one in **Tracked section**, with separate **Entry stats**.
- Render untracked text files as **Known text stats** with all lines added: `+N/-0`.
- Render binary or non-line-oriented changes as **Unknown stats**: `+?/-?`; do not render `binary` or `+0/-0`.
- Render **Rename display** as `old/path -> new/path` and sort by destination path.
- Hide empty **Section** values; use **Clean repository output** only when no sections contain entries.
- Use **Color mode** exactly as `--color=auto|always|never`, default `auto`.
- Use **Deterministic ANSI 256-color styling**; do not add theming or truecolor requirements.
- Enforce **Ignored file exclusion**; do not add ignored-file output in first-version tasks.
- Treat submodules as **Submodule path-level change** only; do not inspect submodule internals.
- Use **Temporary Git repository test** for repository behavior and **Rendered output snapshot** for layout/color behavior.
- Attach follow-up implementation tasks to **PRD epic** `gs-y9o`.

## Relationships

- **`gs`** prints one **Enhanced status view** for the current repository.
- **Enhanced status view** contains one **Branch header** and zero or more visible **Section** values.
- A **Section** contains zero or more **Entry** values; empty sections are hidden.
- An **Entry** contains one **Git-letter status symbol**, one **Display path**, and one **Entry stats** value.
- **Entry stats** are either **Known text stats** or **Unknown stats**.
- A **Partially staged file** produces one **Staged section** entry and one **Tracked section** entry.
- **Rename display** is a kind of **Display path**.
- **Upstream divergence** is rendered inside the **Branch header**.
- **Repository inspector**, **Change model**, **Diff/stat calculator**, and **Renderer** live behind the **Library-plus-thin-binary** architecture.
- **Temporary Git repository test** validates repository behavior; **Rendered output snapshot** validates renderer behavior.
- **PRD epic** `gs-y9o` is the parent/source for follow-up implementation tasks.

## Ambiguities

| Ambiguous term | Problem | Canonical decision |
|---|---|---|
| status | Can mean standard `git status`, internal status flags, or product output. | Use **Enhanced status view** for product output and **Git-letter status symbol** for entry markers. |
| tracked | Can mean any Git-tracked file or only unstaged tracked changes. | Use **Tracked section** only for unstaged worktree changes to tracked files. |
| file count | Partially staged files can appear twice, so file count may differ from rendered rows. | Use **Section count** for rendered entry count. |
| filepath | Renames need `old -> new`, not a single path. | Use **Display path** when rendering/sorting output rows. |
| changed lines | Can imply total changes or split additions/deletions. | Use **Entry stats**, **Known text stats**, or **Unknown stats**. |
| binary | Can describe file type, diff handling, or output string. | Use **Unknown stats** for rendered binary/unknown line changes. |
| branch summary | Could include verbose prose or labels. | Use **Branch header** with compact `branch ↑ahead ↓behind` format. |
| clean | Could mean no output, no file changes, or no branch divergence. | Use **Clean repository output** when no sections contain entries; branch header still renders. |
| color | Could mean theme, auto-detection, or exact palette. | Use **Color mode** for CLI behavior and **Deterministic ANSI 256-color styling** for palette. |
| submodule changes | Could include recursive child repo status or parent pointer changes. | Use **Submodule path-level change** only. |
| tests | Could mean unit, integration, snapshot, or mocked tests. | Use **Temporary Git repository test** for Git behavior and **Rendered output snapshot** for renderer output. |
