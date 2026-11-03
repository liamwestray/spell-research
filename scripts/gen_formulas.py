#!/usr/bin/env python3
"""Regenerate src/formulas.rs from the wiki scroll tables.

Inputs (both checked in):
  data/scroll-tables.tsv   — the VIII scroll tables from asheron.fandom.com
                             (VIII_{Creature,War,Life,Item,Void}_Scrolls),
                             transcribed 2026-11-02: school, scroll, quill,
                             ink, glyph.
  data/spell-table-dump.txt — the server's spell table, used only to pick the
                             spelling each spell's name actually has here.

A wiki scroll "Inscription of X" is the scroll item; the spell it teaches is
"Incantation of X", or "Aura of Incantation of X" for the weapon auras. The
generator keeps a recipe only when its spell exists on this server under one
of those names, so the table never offers a scroll the character cannot
learn.

    python3 scripts/gen_formulas.py > src/formulas.rs
"""

import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def server_names():
    names = set()
    with open(os.path.join(ROOT, "data", "spell-table-dump.txt")) as handle:
        for line in handle:
            cut = line.find("0x")
            if cut < 0:
                continue
            m = re.match(r"^\s*(\d+)\s+school (\d+)\s+(.*?)\s*$", line[:cut])
            if m:
                names.add(m.group(3))
    return names


def variant(cell):
    for prefix in ("Quill of ", "Ink of ", "Glyph of "):
        if cell.startswith(prefix):
            return cell[len(prefix):].lower()
    if cell.lower().endswith(" ink"):  # Alacritous Ink, Parabolic Ink
        return cell[:-4].lower()
    raise ValueError(f"unrecognised component name: {cell!r}")


def candidates(scroll):
    core = re.sub(r"^Inscription of ", "", scroll.replace(" (Scroll)", "").strip())
    if core.startswith("Aura of "):
        effect = core[len("Aura of "):]
        return [f"Aura of Incantation of {effect}", f"Incantation of {effect}"]
    return [
        f"Incantation of {core}",
        f"Aura of Incantation of {core}",
        f"{core} Incantation",
    ]


def component_sig(line):
    """The formula-component string of a dump line, for spotting two ids
    that are the same spell under different names."""
    cut = line.find("0x")
    return line[cut:].strip() if cut >= 0 else ""


def load_spell_signatures():
    """name -> set of component signatures, from the server dump. A spell
    name whose only signature equals a `name Self`/`name Other` sibling's
    is a legacy duplicate of that sibling."""
    sigs = {}
    with open(os.path.join(ROOT, "data", "spell-table-dump.txt")) as handle:
        for line in handle:
            cut = line.find("0x")
            if cut < 0:
                continue
            m = re.match(r"^\s*(\d+)\s+school (\d+)\s+(.*?)\s*$", line[:cut])
            if not m:
                continue
            sigs.setdefault(m.group(3), set()).add(component_sig(line))
    return sigs


def is_legacy_prefix(spell, sigs):
    """True when `spell` is a bare weapon-aura name the server only carries
    as a duplicate of its `… Self` sibling with the same formula.

    The weapon auras (Spirit Drinker, Blood Drinker, …) exist twice: once
    under the proper `… Self`/`… Other` names and once — for some — under a
    truncated `Aura of Incantation of X` with a formula identical to the
    Self one (`Aura of Incantation of Spirit Drinker` id 5182 duplicates
    `… Spirit Drinker Self` id 4414). A character learns the properly-named
    spell, so a recipe bound to the bare name never reads known.

    This is deliberately narrow: only an `Aura of Incantation of …` name
    whose `… Self` sibling shares its exact formula. A war spell that
    merely has empowered copies (Acid Stream vs. Acid Stream with 300
    Spellpower) is a real, learnable spell and is kept."""
    if not spell.startswith("Aura of Incantation of "):
        return False
    mine = sigs.get(spell)
    if not mine:
        return False
    self_sibling = spell + " Self"
    return sigs.get(self_sibling) == mine


def main():
    names = server_names()
    sigs = load_spell_signatures()
    entries = {}
    dropped = []
    with open(os.path.join(ROOT, "data", "scroll-tables.tsv")) as handle:
        for line in handle:
            school, scroll, quill, ink, glyph = line.rstrip("\n").split("\t")
            spell = next(
                (c for c in candidates(scroll)
                 if c in names and not is_legacy_prefix(c, sigs)),
                None,
            )
            if spell is None:
                dropped.append(scroll)
                continue
            triple = (variant(quill), variant(ink), variant(glyph))
            if spell in entries and entries[spell] != triple:
                sys.exit(f"conflicting recipes for {spell}")
            entries[spell] = triple

    quills = sorted({q for q, _, _ in entries.values()})
    inks = sorted({i for _, i, _ in entries.values()})

    out = sys.stdout
    out.write('''//! The level VIII recipes: which incantation exists, and what crafts it.
//!
//! This table is the plugin's only knowledge of what combines into what, and
//! it is deliberately hand-carried from the wiki rather than read from the
//! game. The client's spell table cannot answer this question: the formula
//! components it names are the old reagent costs for *learning* a spell
//! (tapers, herbs, powders, talismans), not the quill, ink and glyph that
//! *craft* its scroll, and its spell "level" is a heuristic the client
//! guesses from the component count — which is why it claims hundreds of
//! non-VIII spells. The crafting mapping is server-side; the community's
//! record of it is the Spell Research page and its per-school scroll tables
//! on asheron.fandom.com.
//!
//! `scripts/gen_formulas.py` regenerates this file from `data/scroll-tables.tsv`
//! (the wiki tables) and `data/spell-table-dump.txt` (so each entry carries
//! the spelling this server uses). A recipe whose spell is absent from the
//! server is left out: there is nothing to research.

/// One researchable incantation: the spell's name as the server spells it,
/// and the lowercased variants of the quill, ink and glyph that make its
/// scroll, matching [`crate::component::classify`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Formula {
    /// The spell the scroll teaches, named as the spell table names it.
    pub spell: &'static str,
    /// Which quill: see [`QUILLS`].
    pub quill: &'static str,
    /// Which ink: see [`INKS`].
    pub ink: &'static str,
    /// Which glyph.
    pub glyph: &'static str,
}

/// The four quills, from the Spell Research page. Nothing else is a quill.
pub const QUILLS: &[&str] = &[''')
    out.write(", ".join(f'"{q}"' for q in quills))
    out.write('''];

/// The nine inks, from the Spell Research page.
pub const INKS: &[&str] = &[''')
    out.write(", ".join(f'"{i}"' for i in inks))
    out.write('''];

/// The scarab every infusion consumes.
pub const SCARAB: &str = "mana";

/// Every recipe, in spell-name order.
pub const FORMULAS: &[Formula] = &[
''')
    for spell in sorted(entries):
        q, i, g = entries[spell]
        out.write(
            f'    Formula {{ spell: "{spell}", quill: "{q}", ink: "{i}", '
            f'glyph: "{g}" }},\n'
        )
    out.write('''];

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn the_table_is_in_spell_name_order() {
        let names: Vec<&str> = FORMULAS.iter().map(|f| f.spell).collect();
        let mut sorted = names.clone();
        sorted.sort_unstable();
        assert_eq!(names, sorted);
    }

    #[test]
    fn no_spell_appears_twice() {
        let unique: BTreeSet<&str> = FORMULAS.iter().map(|f| f.spell).collect();
        assert_eq!(unique.len(), FORMULAS.len());
    }

    #[test]
    fn every_component_is_from_the_pages_vocabulary() {
        for f in FORMULAS {
            assert!(QUILLS.contains(&f.quill), "{}: {}", f.spell, f.quill);
            assert!(INKS.contains(&f.ink), "{}: {}", f.spell, f.ink);
            assert!(!f.glyph.is_empty(), "{}", f.spell);
        }
    }
}
''')
    print(
        f"// {len(entries)} recipes kept, {len(dropped)} wiki scrolls dropped "
        f"(spell not on this server): {', '.join(dropped)}",
        file=sys.stderr,
    )


if __name__ == "__main__":
    main()