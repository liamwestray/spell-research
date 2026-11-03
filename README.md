# Spell Research

Tracks and automates level VIII spell scroll crafting for
[openac-rust](../openac-rust). A WebAssembly plugin; the client runs it on the
session's own thread.

```
cargo test                      # the classification logic, natively
scripts/build.sh --test         # test, build wasm, install into the client
```

In game: `/plugin reload all`, then `/spellresearch` (`full` for one line per
spell).

## What it does

One row per researchable level VIII incantation: its icon, name, and its
three components plus the mana scarab, each drawn as the real item icon when
the pack holds one and left blank (with the name greyed) when it does not.
Known spells draw green, and the **Craft** button on the right is live only
when the whole chain — quill, mana scarab, ink, glyph — is in hand.

Tabs across the top scope the window by school — **All | Creature | Item |
Life | War/Void** — and they scope *crafting* too: **Craft all** only ever
queues the rows the current tab shows, so you can burn through one school at
a time. Switching tabs drops queued spells and stops a running job as soon
as its current combine settles — the plugin never crafts a spell that is not
on screen. Filters for unknown-only and ready-only compose with the tab, a
tally in the header counts the tab's rows, and a pack line counts each
component family.

Crafting is the retail chain, three `apply_item` calls:

```
quill + mana scarab  ->  infused quill
infused quill + ink  ->  combination
combination + glyph  ->  level VIII scroll
```

All four are consumed on success. Failure costs nothing and there is **no skill
requirement** for level VIII crafting, so there is deliberately no skill gate on
the button; the 400 figure people remember is casting difficulty, not crafting.

## Why the recipes are a table, not the spell table

The client's spell table cannot say what crafts a level VIII scroll. Its
formula components are the old reagent costs for **learning** a spell —
`Orange Taper`, `Hyssop`, `Copper Scarab`, `Birch Talisman` — and its spell
"level" is a heuristic the client guesses from the component count, which is
why an earlier build of this plugin claimed 592 "level VIII" spells and drew
amber rows for tapers. The quill/ink/glyph mapping is server-side.

So `src/formulas.rs` carries the community's record of it: every recipe from
the Spell Research page's Known Formulas, expanded through the wiki's
per-school scroll tables, restricted to the spells this server actually has.
199 researchable incantations, and only the page's materials — four quills,
nine inks, the glyphs those recipes use, and the mana scarab — classify as
components at all. `scripts/gen_formulas.py` regenerates the table from
`data/scroll-tables.tsv` and `data/spell-table-dump.txt` if the server's
names ever change.

`src/component.rs` sorts a resolved name into its role (`Quill of Benevolence`,
`Alacritous Ink` / `Ink of Conveyance`, `Glyph of Strength`, `Mana Scarab`)
and rejects any variant the page does not name, so a copper scarab or a
glyph no recipe wants never counts toward a craft.

## Status: built, untested against a live server

41 unit tests, clippy clean, and the window's markup is parsed in a test
against the same binding list the plugin registers — so a misspelled element,
a missing required attribute or an undeclared binding fails the build instead
of leaving the window silently absent. (An unknown *optional* attribute is
ignored by the parser and will not be caught.)

The table itself is transcribed data, so the check that matters is still the
live one: whether this server accepts the wiki's combinations. Say
`/spellresearch` for the counts it matched, and craft one scroll before
trusting "Craft all".

## Layout

| File             | Holds                                                           |
| ---------------- | --------------------------------------------------------------- |
| `formulas.rs`  | the recipe table, generated from the wiki (see `scripts/`)    |
| `component.rs` | name → role, gated to the page's vocabulary                    |
| `recipe.rs`    | a spell and its requirements, joined to the spell table by name |
| `stock.rs`     | the pack, counted by role and variant                           |
| `craft.rs`     | the three-combine state machine, pure over pack snapshots       |
| `view.rs`      | the markup, the binding list, and the rows                      |
| `lib.rs`       | wiring: read, craft, refresh                                    |

Crafting is driven from `tick`, one combine at a time, and the product of each
is found by diffing the pack before and after — its client id cannot be known
in advance. A combine the server refuses consumes nothing, so a stopped job
leaves the components where they were.

## Next

1. Run `/spellresearch` and check the matched count against the server.
2. Craft one scroll by hand and confirm the chain matches.
3. Then trust "Craft all".

## Notes

- Only five permissions exist (`chat`, `network`, `listen`, `profiles`,
  `clipboard`); reading items and spells is not gated, so this declares `chat`
  alone, for its reports.
- The plugin lives beside the client repo, not inside it, so `openac-rust`
  stays clean.
