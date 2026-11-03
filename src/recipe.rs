//! A level VIII scroll recipe: the spell, and what crafts it.
//!
//! The recipes come from [`crate::formulas`], a table transcribed from the
//! wiki's Spell Research page and its per-school scroll tables. The game's
//! spell table is consulted only to find each spell by name — its id (for
//! the known check), its icon and its school. Its formula components are
//! the old reagent costs for *learning* a spell (tapers, herbs, powders,
//! talismans), not the quill, ink and glyph that *craft* its scroll, so
//! they are never read.
//!
//! Requirements are matched by `(role, variant)` rather than by weenie
//! class, because both sides of the comparison — the recipe and the item
//! in the pack — arrive with a name. That keeps the whole model in one
//! place: if the classification in `component.rs` turns out to be wrong
//! about some family, correcting it corrects the recipes and the inventory
//! count together, and nothing has to be re-derived by hand.

use crate::component::Role;
use crate::formulas::{Formula, SCARAB};
use openac_plugin::magic::SpellInfo;

/// One thing a scroll needs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Requirement {
    /// Quill, scarab, ink or glyph.
    pub role: Role,
    /// Which one, lowercased; `None` when any of the role serves.
    ///
    /// Every requirement the table produces names its variant — the
    /// recipe is complete, and the scarab is always a mana scarab — but
    /// the shape stays open so a future rule ("any ink of this family")
    /// needs no new type.
    pub variant: Option<String>,
}

/// A researchable spell and how to craft its scroll.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recipe {
    /// The spell it produces: the lowest of the table's ids for this name,
    /// used as the row's identity.
    pub spell_id: u32,
    /// Every id the spell table lists under this name. The server can carry
    /// the same spell more than once (the retail id plus later duplicates),
    /// and a character who knows any of them knows the spell.
    pub ids: Vec<u32>,
    /// Its name, as the server spells it.
    pub name: String,
    /// The magic skill it is cast with (31 creature, 32 item, 33 life,
    /// 34 war, 43 void).
    pub school: u32,
    /// Its icon.
    pub icon_id: u32,
    /// Which quill, lowercased.
    pub quill: &'static str,
    /// Which ink, lowercased.
    pub ink: &'static str,
    /// Which glyph, lowercased.
    pub glyph: &'static str,
}

impl Recipe {
    /// The recipe that makes `formula`'s scroll, from every table entry
    /// sharing the spell's name. `None` when the server has no such spell.
    #[must_use]
    pub fn new(formula: &Formula, mut spells: Vec<&SpellInfo>) -> Option<Self> {
        spells.sort_by_key(|spell| spell.spell_id);
        let first = spells.first()?;
        Some(Self {
            spell_id: first.spell_id,
            ids: spells.iter().map(|spell| spell.spell_id).collect(),
            name: first.name.clone(),
            school: first.school,
            icon_id: first.icon_id,
            quill: formula.quill,
            ink: formula.ink,
            glyph: formula.glyph,
        })
    }

    /// Whether the character knows the spell under any of its ids.
    #[must_use]
    pub fn is_known(&self, known: &dyn Fn(u32) -> bool) -> bool {
        self.ids.iter().any(|id| known(*id))
    }

    /// What the scroll needs, in combining order: the quill, the scarab,
    /// the ink and the glyph.
    #[must_use]
    pub fn requirements(&self) -> Vec<Requirement> {
        [
            (Role::Quill, self.quill),
            (Role::Scarab, SCARAB),
            (Role::Ink, self.ink),
            (Role::Glyph, self.glyph),
        ]
        .into_iter()
        .map(|(role, variant)| Requirement {
            role,
            variant: Some(variant.to_owned()),
        })
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spell(id: u32) -> SpellInfo {
        SpellInfo {
            spell_id: id,
            name: "Incantation of Alchemy Mastery Self".to_owned(),
            school: 31,
            icon_id: 7,
            ..Default::default()
        }
    }

    fn formula() -> Formula {
        Formula {
            spell: "Incantation of Alchemy Mastery Self",
            quill: "introspection",
            ink: "formation",
            glyph: "alchemy",
        }
    }

    #[test]
    fn a_recipe_wants_its_three_components_and_a_mana_scarab() {
        let r = Recipe::new(&formula(), vec![&spell(4506)]).expect("one entry");
        let wants = r.requirements();
        let roles: Vec<Role> = wants.iter().map(|w| w.role).collect();
        assert_eq!(
            roles,
            vec![Role::Quill, Role::Scarab, Role::Ink, Role::Glyph],
            "the order the crafting combines them"
        );
        let variants: Vec<&str> = wants
            .iter()
            .map(|w| w.variant.as_deref().unwrap_or_default())
            .collect();
        assert_eq!(variants, ["introspection", "mana", "formation", "alchemy"]);
    }

    #[test]
    fn a_spell_known_under_any_of_its_ids_counts_as_known() {
        // The server lists the same spell under several ids; whichever one
        // the character learned, the row must draw green.
        let low = spell(4522);
        let high = spell(4628);
        let r = Recipe::new(&formula(), vec![&high, &low]).expect("two entries");
        assert_eq!(r.spell_id, 4522, "the lowest id is the row's identity");
        assert_eq!(r.ids, vec![4522, 4628]);
        assert!(r.is_known(&|id| id == 4628), "knowing the duplicate counts");
        assert!(r.is_known(&|id| id == 4522), "knowing the base counts");
        assert!(!r.is_known(&|id| id == 9999), "an unrelated id does not");
    }
}
