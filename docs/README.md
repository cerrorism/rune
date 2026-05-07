# Rune Docs Index

This is the first file to read at the start of any session. It tells you what to read next and what to update before you stop.

---

## Restore context at the start of a session (~5 minutes)

1. **`progress.md`** — Read `last_session_summary` and find the first unchecked task under the active phase. This is your anchor.
2. **`design/phaseN-*.md`** (active phase only) — Confirm the module map and data types match what's in the code.
3. **`findings.md`** — Scan the active phase's section only. The goal: don't walk into a known trap.

That's it. `project.md` and `decisions.md` are for onboarding or when a major decision is being made — not for every session.

---

## Update at the end of a session

1. **`progress.md`** — Check off completed tasks. Update `last_session_summary` with one sentence. Add any new tasks discovered.
2. **`findings.md`** — Add an entry for any mistake made (`[RUST]`), design reversed (`[REVERSAL]`), or debate settled (`[RESOLVED]`). If nothing notable happened, add nothing.
3. **`journal.md`** — Write a narrative entry if something genuinely clicked or confused. Optional — don't force it. A good signal: if you found yourself explaining something and the explanation helped you understand it better, it belongs here.
4. **`decisions.md`** — Add an ADR entry if a new architectural decision was made.

Design docs (`design/phaseN-*.md`) are updated *during* the session as decisions are made, not at the end.

---

## File map

| File | Purpose | Update frequency |
|---|---|---|
| `project.md` | Project charter: phases, non-goals, central abstraction | Rarely — only when goals change |
| `progress.md` | Live task checklist + session log | Every session |
| `findings.md` | Errors, reversals, resolved debates | Most sessions |
| `decisions.md` | Architecture decisions with rationale | When a new decision is made |
| `journal.md` | Narrative Rust learning log; raw blog material | When something clicks |
| `design/phase1-scanner.md` | Phase 1 design contract | While Phase 1 is active |
| `design/phase2-s3.md` | Phase 2 design (stub until Phase 1 is done) | When Phase 2 begins |
| `design/phase3-table.md` | Phase 3 design (stub) | When Phase 3 begins |
| `design/phase4-query.md` | Phase 4 design (stub) | When Phase 4 begins |
| `design/phase5-distributed.md` | Phase 5 design (stub) | When Phase 5 begins |
