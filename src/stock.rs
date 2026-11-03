//! What is in the pack, counted by role and variant.
//!
//! The count is keyed the same way a [`crate::recipe::Requirement`] is
//! matched, so "do I have it" is one comparison and both sides come from the
//! same classifier. Items are counted by their name as the client reports it,
//! which covers a stack and covers an item tucked inside a pack: everything
//! the character owns arrives in one list with its container already resolved.

use std::collections::BTreeMap;

use crate::component::{Role, classify};
use crate::recipe::Requirement;
use openac_plugin::items::InventoryItem;

/// A component the character carries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Held {
    /// The client's id for this item, to use in a craft.
    pub object_id: u32,
    /// Its weenie class.
    pub weenie_class_id: u32,
    /// Its name, as the client spells it.
    pub name: String,
    /// Its icon's render surface; 0 until the client has described the
    /// item, which it does for everything in the pack.
    pub icon_id: u32,
    /// What it contributes.
    pub role: Role,
    /// Which one, lowercased.
    pub variant: String,
    /// How many in the stack; 1 for a single item.
    pub count: u32,
}

impl Held {
    /// Its icon's render surface; 0 until filled.
    #[must_use]
    pub const fn icon_id(&self) -> u32 {
        self.icon_id
    }
}

/// Everything of a component family the character owns.
#[derive(Debug, Default)]
pub struct Stock {
    /// By role and variant, oldest object first so a craft picks a stable
    /// item rather than one that moves around.
    by_key: BTreeMap<(Role, String), Vec<Held>>,
    /// By role alone, for a requirement that accepts any variant.
    by_role: BTreeMap<Role, Vec<Held>>,
}

impl Stock {
    /// Count `items`, keeping only those that classify as a component.
    #[must_use]
    pub fn from_items(items: &[InventoryItem]) -> Self {
        let mut stock = Self::default();
        for item in items {
            let c = classify(&item.name);
            if c.role == Role::Other {
                continue;
            }
            let held = Held {
                object_id: item.object_id,
                weenie_class_id: item.weenie_class_id,
                name: item.name.clone(),
                icon_id: 0,
                role: c.role,
                variant: c.variant,
                count: u32::try_from(item.stack_size.max(1)).unwrap_or(1),
            };
            stock
                .by_key
                .entry((held.role, held.variant.clone()))
                .or_default()
                .push(held.clone());
            stock.by_role.entry(held.role).or_default().push(held);
        }
        stock
    }

    /// How many satisfy `want`.
    #[must_use]
    pub fn count(&self, want: &Requirement) -> u32 {
        match &want.variant {
            Some(variant) => self
                .by_key
                .get(&(want.role, variant.to_lowercase()))
                .map(|held| held.iter().map(|h| h.count).sum())
                .unwrap_or(0),
            None => self
                .by_role
                .get(&want.role)
                .map(|held| held.iter().map(|h| h.count).sum())
                .unwrap_or(0),
        }
    }

    /// Whether the pack holds at least `need` of `want`.
    #[must_use]
    pub fn holds(&self, want: &Requirement, need: u32) -> bool {
        self.count(want) >= need
    }

    /// An item that satisfies `want`, for starting a craft.
    #[must_use]
    pub fn find(&self, want: &Requirement) -> Option<&Held> {
        match &want.variant {
            Some(variant) => self
                .by_key
                .get(&(want.role, variant.to_lowercase()))
                .and_then(|held| held.first()),
            None => self.by_role.get(&want.role).and_then(|held| held.first()),
        }
    }

    /// How many of each role the character carries.
    #[must_use]
    pub fn totals(&self) -> BTreeMap<&'static str, u32> {
        let mut out = BTreeMap::new();
        for (role, held) in &self.by_role {
            *out.entry(role.label()).or_insert(0u32) += held.iter().map(|h| h.count).sum::<u32>();
        }
        out
    }

    /// Fill each held component's icon from `icon`, which answers an
    /// object id with its render surface (0 for none). The inventory list
    /// does not carry icons; the world's object table does, and it knows
    /// everything in the pack.
    pub fn fill_icons(&mut self, mut icon: impl FnMut(u32) -> u32) {
        for held in self.by_key.values_mut().chain(self.by_role.values_mut()) {
            for h in held {
                h.icon_id = icon(h.object_id);
            }
        }
    }

    /// The icon of one component that satisfies `want`, 0 when the pack
    /// lacks it or the client has not described it.
    #[must_use]
    pub fn icon(&self, want: &Requirement) -> u32 {
        self.find(want).map_or(0, Held::icon_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(object_id: u32, name: &str, stack: i32) -> InventoryItem {
        InventoryItem {
            object_id,
            weenie_class_id: 0,
            name: name.to_owned(),
            item_type: 0,
            container_id: 0,
            wielder_id: 0,
            valid_locations: 0,
            equipped_location: 0,
            useability: 0,
            target_type: 0,
            public_flags: 0,
            stack_size: stack,
            structure: 1,
            maximum_structure: 1,
            spell_id: 0,
            pet_class: 0,
            summoning_mastery: 0,
            shared_cooldown_id: 0,
            cooldown_seconds: 0.0,
            ..Default::default()
        }
    }

    fn want(role: Role, variant: Option<&str>) -> Requirement {
        Requirement {
            role,
            variant: variant.map(str::to_owned),
        }
    }

    #[test]
    fn a_stack_counts_its_whole_size() {
        let stock = Stock::from_items(&[item(1, "Mana Scarab", 12)]);
        assert_eq!(stock.count(&want(Role::Scarab, None)), 12);
    }

    #[test]
    fn separate_items_add_up() {
        let stock = Stock::from_items(&[
            item(1, "Glyph of Strength", 1),
            item(2, "Glyph of Strength", 1),
        ]);
        assert_eq!(stock.count(&want(Role::Glyph, Some("strength"))), 2);
    }

    #[test]
    fn one_variant_does_not_stand_in_for_another() {
        let stock = Stock::from_items(&[item(1, "Quill of Benevolence", 3)]);
        assert_eq!(stock.count(&want(Role::Quill, Some("benevolence"))), 3);
        assert_eq!(stock.count(&want(Role::Quill, Some("infliction"))), 0);
    }

    #[test]
    fn any_variant_serves_an_open_requirement() {
        let stock = Stock::from_items(&[item(1, "Mana Scarab", 2)]);
        assert!(stock.holds(&want(Role::Scarab, None), 2));
        assert!(!stock.holds(&want(Role::Scarab, None), 3));
    }

    #[test]
    fn things_that_are_not_components_are_ignored() {
        let stock = Stock::from_items(&[item(1, "Shadow Armor", 1), item(2, "Ruby Powder", 4)]);
        assert_eq!(stock.count(&want(Role::Quill, None)), 0);
        assert_eq!(stock.count(&want(Role::Glyph, None)), 0);
        assert!(stock.totals().is_empty(), "nothing of a role at all");
    }

    #[test]
    fn find_returns_an_item_that_matches() {
        let stock = Stock::from_items(&[item(42, "Ink of Conveyance", 1)]);
        let found = stock
            .find(&want(Role::Ink, Some("conveyance")))
            .expect("held");
        assert_eq!(found.object_id, 42);
        assert_eq!(found.name, "Ink of Conveyance");
    }

    #[test]
    fn totals_count_each_role_once() {
        let stock = Stock::from_items(&[
            item(1, "Quill of Benevolence", 2),
            item(2, "Quill of Infliction", 1),
            item(3, "Mana Scarab", 5),
        ]);
        let totals = stock.totals();
        assert_eq!(totals.get("quill"), Some(&3));
        assert_eq!(totals.get("scarab"), Some(&5));
    }

    #[test]
    fn filling_icons_reaches_both_indexes_and_a_missing_want_has_none() {
        let mut stock = Stock::from_items(&[item(7, "Glyph of Strength", 1)]);
        stock.fill_icons(|id| if id == 7 { 0x0601_2345 } else { 0 });
        let held = want(Role::Glyph, Some("strength"));
        assert_eq!(stock.icon(&held), 0x0601_2345);
        assert_eq!(
            stock.find(&held).map(Held::icon_id),
            Some(0x0601_2345),
            "by_role sees the fill too"
        );
        assert_eq!(
            stock.icon(&want(Role::Glyph, Some("flame"))),
            0,
            "a component not held has no icon"
        );
    }
}
