## Issue Tracking

This project uses **bd (beads)** for issue tracking.
Run `bd prime` for workflow context, or install hooks (`bd hooks install`) for auto-injection.

**Quick reference:**
- `bd ready` - Find unblocked work
- `bd create "Title" --type task --priority 2` - Create issue
- `bd close <id>` - Complete work

For full workflow details: `bd prime`

## Terminology

Before changing domain behavior, read `AGENT_LEXICON.md`.
Use canonical terms from `AGENT_LEXICON.md` in code, docs, task descriptions, and agent outputs.
Do not introduce synonyms for existing concepts unless updating `AGENT_LEXICON.md` first.
Do not duplicate the full lexicon inside `AGENTS.md`.
