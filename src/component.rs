//! Sorting a spell component into the role it plays in scroll crafting.
//!
//! Only the materials the Spell Research page names classify as components:
//! the four quills, the nine inks, the glyphs the recipe table actually uses,
//! and the mana scarab. Everything else — including the old reagent costs
//! for *learning* a spell (`Copper Scarab`, `Orange Taper`, `Hyssop`) — is
//! `Other`, so it never counts toward a craft.

use crate::formulas::{INKS, QUILLS, SCARAB};

/// What one component contributes to a scroll.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Role {
    /// The empty quill, before the scarab infuses it.
    Quill,
    /// The scarab that infuses a quill, and the casting focus for VIII.
    Scarab,
    /// The ink combined with the infused quill.
    Ink,
    /// The glyph that decides which scroll the combination becomes.
    Glyph,
    /// Anything else: a learning cost, a component no recipe uses, or a
    /// name this does not know.
    Other,
}

impl Role {
    /// Its name, for the UI.
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::Quill => "quill",
            Self::Scarab => "scarab",
            Self::Ink => "ink",
            Self::Glyph => "glyph",
            Self::Other => "other",
        }
    }
}

/// A component's role and, for the three named roles, which one it is:
/// "Strength" for `Glyph of Strength`, "Alacritous" for `Alacritous Ink`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Classified {
    /// What it contributes.
    pub role: Role,
    /// The distinguishing word, lowercased; empty for `Other`.
    pub variant: String,
}

/// Sort `name` into a role, accepting only the page's vocabulary.
///
/// The retail names are consistent enough to read the role off directly:
/// `Quill of Benevolence`, `Ink of Conveyance`, `Alacritous Ink`,
/// `Glyph of Strength`, `Mana Scarab` — but a name in a known shape is not
/// enough: `Quill of Fortitude` and `Copper Scarab` look right and are not
/// materials of any recipe, so an unrecognised variant is `Other`.
#[must_use]
pub fn classify(name: &str) -> Classified {
    let trimmed = name.trim();
    let lower = trimmed.to_lowercase();

    if let Some(rest) = strip_prefix_any(&lower, trimmed, &["quill of "]) {
        return gated(named(Role::Quill, rest), QUILLS);
    }
    if let Some(rest) = strip_prefix_any(&lower, trimmed, &["ink of "]) {
        return gated(named(Role::Ink, rest), INKS);
    }
    // `Alacritous Ink`, `Parabolic Ink`: the word comes first.
    if let Some(rest) = lower.strip_suffix(" ink") {
        return gated(named(Role::Ink, &trimmed[..rest.len()]), INKS);
    }
    if let Some(rest) = strip_prefix_any(&lower, trimmed, &["glyph of "]) {
        let c = named(Role::Glyph, rest);
        return if known_glyph(&c.variant) { c } else { other() };
    }
    if lower == "mana scarab" {
        return named(Role::Scarab, SCARAB);
    }
    Classified {
        role: Role::Other,
        variant: String::new(),
    }
}

/// Whether some recipe in the table wants this glyph.
fn known_glyph(variant: &str) -> bool {
    crate::formulas::FORMULAS.iter().any(|f| f.glyph == variant)
}

/// Keep `c` only if its variant is in `vocabulary`.
fn gated(c: Classified, vocabulary: &[&str]) -> Classified {
    if vocabulary.contains(&c.variant.as_str()) {
        c
    } else {
        other()
    }
}

/// Capitalise a lowercased variant for display: `two handed combat` becomes
/// `Two Handed Combat`. The game's own name is not kept because the row shows
/// three components in a narrow column, where the family word is what matters.
#[must_use]
pub fn title(variant: &str) -> String {
    variant
        .split(' ')
        .map(|word| match word.split_once(char::is_whitespace) {
            Some(_) => word.to_owned(),
            None => match word.get(..1) {
                Some(first) => match word.get(1..) {
                    Some(rest) => format!("{}{}", first.to_uppercase(), rest),
                    None => first.to_uppercase(),
                },
                None => String::new(),
            },
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// A `Role::Other` with no variant.
fn other() -> Classified {
    Classified {
        role: Role::Other,
        variant: String::new(),
    }
}

/// A classified component whose variant is `name`, lowercased.
fn named(role: Role, name: &str) -> Classified {
    let variant = name.trim().to_lowercase();
    if variant.is_empty() {
        return other();
    }
    Classified { role, variant }
}

/// Match `lower` against each prefix, and return the rest of `original`.
///
/// The prefixes are lowercase and the rest is cut out of `original`, so the
/// variant keeps the item's own capitalisation before it is lowercased.
fn strip_prefix_any<'a>(lower: &'a str, original: &'a str, prefixes: &[&str]) -> Option<&'a str> {
    for prefix in prefixes {
        if lower.starts_with(prefix) {
            return Some(&original[prefix.len()..]);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quills_are_quills() {
        for name in [
            "Quill of Benevolence",
            "Quill of Extraction",
            "Quill of Infliction",
            "Quill of Introspection",
        ] {
            let c = classify(name);
            assert_eq!(c.role, Role::Quill, "{name}");
        }
        assert_eq!(classify("Quill of Benevolence").variant, "benevolence");
    }

    #[test]
    fn inks_come_in_both_orders() {
        assert_eq!(
            classify("Ink of Conveyance"),
            Classified {
                role: Role::Ink,
                variant: "conveyance".to_owned()
            }
        );
        assert_eq!(
            classify("Alacritous Ink"),
            Classified {
                role: Role::Ink,
                variant: "alacritous".to_owned()
            }
        );
        assert_eq!(classify("Parabolic Ink").variant, "parabolic");
    }

    #[test]
    fn glyphs_carry_the_spell_choice() {
        assert_eq!(classify("Glyph of Strength").variant, "strength");
        assert_eq!(classify("Glyph of Heavy Weapons").variant, "heavy weapons");
        assert_eq!(classify("Glyph of War Magic").role, Role::Glyph);
    }

    #[test]
    fn scarabs_are_the_focus() {
        assert_eq!(classify("Mana Scarab").role, Role::Scarab);
        assert_eq!(classify("Mana Scarab").variant, "mana");
    }

    #[test]
    fn learning_costs_are_not_components() {
        // The old reagent costs for learning a spell. They share the shapes
        // of real components and must not pass for them.
        for name in [
            "Copper Scarab",
            "Lead Scarab",
            "Platinum Scarab",
            "Orange Taper",
            "Hyssop",
            "Birch Talisman",
            "Powdered Onyx",
        ] {
            assert_eq!(classify(name).role, Role::Other, "{name}");
        }
    }

    #[test]
    fn a_variant_outside_the_pages_vocabulary_is_not_a_component() {
        assert_eq!(classify("Quill of Fortitude").role, Role::Other);
        assert_eq!(classify("Ink of Invention").role, Role::Other);
        assert_eq!(classify("Glyph of Netherlight").role, Role::Other);
    }

    #[test]
    fn anything_else_is_other() {
        // A lower-tier component: the plugin must not guess a role for it.
        assert_eq!(classify("Ruby Powder").role, Role::Other);
        assert_eq!(classify("Glyph").role, Role::Other, "no variant");
        assert_eq!(classify("").role, Role::Other);
        assert_eq!(classify("Quill of").role, Role::Other, "no variant");
    }

    #[test]
    fn surrounding_space_does_not_change_the_answer() {
        assert_eq!(classify("  Glyph of Flame ").variant, "flame");
    }
}
