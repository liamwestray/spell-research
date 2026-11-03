//! The level VIII recipes: which incantation exists, and what crafts it.
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
pub const QUILLS: &[&str] = &["benevolence", "extraction", "infliction", "introspection"];

/// The nine inks, from the Spell Research page.
pub const INKS: &[&str] = &[
    "alacritous",
    "conveyance",
    "direction",
    "formation",
    "nullification",
    "objectification",
    "parabolic",
    "partition",
    "separation",
];

/// The scarab every infusion consumes.
pub const SCARAB: &str = "mana";

/// Every recipe, in spell-name order.
pub const FORMULAS: &[Formula] = &[
    Formula {
        spell: "Aura of Incantation of Blood Drinker Other",
        quill: "benevolence",
        ink: "objectification",
        glyph: "strength",
    },
    Formula {
        spell: "Aura of Incantation of Blood Drinker Self",
        quill: "introspection",
        ink: "objectification",
        glyph: "strength",
    },
    Formula {
        spell: "Aura of Incantation of Defender Other",
        quill: "benevolence",
        ink: "objectification",
        glyph: "melee defense",
    },
    Formula {
        spell: "Aura of Incantation of Defender Self",
        quill: "introspection",
        ink: "objectification",
        glyph: "melee defense",
    },
    Formula {
        spell: "Aura of Incantation of Heart Seeker Other",
        quill: "benevolence",
        ink: "objectification",
        glyph: "coordination",
    },
    Formula {
        spell: "Aura of Incantation of Heart Seeker Self",
        quill: "introspection",
        ink: "objectification",
        glyph: "coordination",
    },
    Formula {
        spell: "Aura of Incantation of Hermetic Link Other",
        quill: "benevolence",
        ink: "objectification",
        glyph: "mana conversion",
    },
    Formula {
        spell: "Aura of Incantation of Hermetic Link Self",
        quill: "introspection",
        ink: "objectification",
        glyph: "mana conversion",
    },
    Formula {
        spell: "Aura of Incantation of Spirit Drinker Other",
        quill: "benevolence",
        ink: "objectification",
        glyph: "focus",
    },
    Formula {
        spell: "Aura of Incantation of Spirit Drinker Self",
        quill: "introspection",
        ink: "objectification",
        glyph: "focus",
    },
    Formula {
        spell: "Aura of Incantation of Swift Killer Other",
        quill: "benevolence",
        ink: "objectification",
        glyph: "quickness",
    },
    Formula {
        spell: "Aura of Incantation of Swift Killer Self",
        quill: "introspection",
        ink: "objectification",
        glyph: "quickness",
    },
    Formula {
        spell: "Incantation of Acid Arc",
        quill: "infliction",
        ink: "parabolic",
        glyph: "corrosion",
    },
    Formula {
        spell: "Incantation of Acid Bane",
        quill: "benevolence",
        ink: "objectification",
        glyph: "corrosion",
    },
    Formula {
        spell: "Incantation of Acid Blast",
        quill: "infliction",
        ink: "partition",
        glyph: "corrosion",
    },
    Formula {
        spell: "Incantation of Acid Lure",
        quill: "infliction",
        ink: "objectification",
        glyph: "corrosion",
    },
    Formula {
        spell: "Incantation of Acid Protection Self",
        quill: "introspection",
        ink: "formation",
        glyph: "corrosion",
    },
    Formula {
        spell: "Incantation of Acid Streak",
        quill: "infliction",
        ink: "alacritous",
        glyph: "corrosion",
    },
    Formula {
        spell: "Incantation of Acid Stream",
        quill: "infliction",
        ink: "direction",
        glyph: "corrosion",
    },
    Formula {
        spell: "Incantation of Acid Volley",
        quill: "infliction",
        ink: "separation",
        glyph: "corrosion",
    },
    Formula {
        spell: "Incantation of Acid Vulnerability Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "corrosion",
    },
    Formula {
        spell: "Incantation of Alchemy Ineptitude Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "alchemy",
    },
    Formula {
        spell: "Incantation of Alchemy Mastery Self",
        quill: "introspection",
        ink: "formation",
        glyph: "alchemy",
    },
    Formula {
        spell: "Incantation of Arcane Benightedness Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "arcane lore",
    },
    Formula {
        spell: "Incantation of Arcane Enlightenment Self",
        quill: "introspection",
        ink: "formation",
        glyph: "arcane lore",
    },
    Formula {
        spell: "Incantation of Arcanum Salvaging Self",
        quill: "introspection",
        ink: "formation",
        glyph: "salvaging",
    },
    Formula {
        spell: "Incantation of Armor Self",
        quill: "introspection",
        ink: "formation",
        glyph: "armor",
    },
    Formula {
        spell: "Incantation of Armor Tinkering Expertise Self",
        quill: "introspection",
        ink: "formation",
        glyph: "armor tinkering",
    },
    Formula {
        spell: "Incantation of Armor Tinkering Ignorance Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "armor tinkering",
    },
    Formula {
        spell: "Incantation of Bafflement Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "focus",
    },
    Formula {
        spell: "Incantation of Blade Arc",
        quill: "infliction",
        ink: "parabolic",
        glyph: "slashing",
    },
    Formula {
        spell: "Incantation of Blade Bane",
        quill: "benevolence",
        ink: "objectification",
        glyph: "slashing",
    },
    Formula {
        spell: "Incantation of Blade Blast",
        quill: "infliction",
        ink: "partition",
        glyph: "slashing",
    },
    Formula {
        spell: "Incantation of Blade Lure",
        quill: "infliction",
        ink: "objectification",
        glyph: "slashing",
    },
    Formula {
        spell: "Incantation of Blade Protection Self",
        quill: "introspection",
        ink: "formation",
        glyph: "slashing",
    },
    Formula {
        spell: "Incantation of Blade Volley",
        quill: "infliction",
        ink: "separation",
        glyph: "slashing",
    },
    Formula {
        spell: "Incantation of Blade Vulnerability Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "slashing",
    },
    Formula {
        spell: "Incantation of Blood Loather",
        quill: "infliction",
        ink: "objectification",
        glyph: "strength",
    },
    Formula {
        spell: "Incantation of Bludgeon Bane",
        quill: "benevolence",
        ink: "objectification",
        glyph: "bludgeoning",
    },
    Formula {
        spell: "Incantation of Bludgeon Lure",
        quill: "infliction",
        ink: "objectification",
        glyph: "bludgeoning",
    },
    Formula {
        spell: "Incantation of Bludgeoning Protection Self",
        quill: "introspection",
        ink: "formation",
        glyph: "bludgeoning",
    },
    Formula {
        spell: "Incantation of Bludgeoning Volley",
        quill: "infliction",
        ink: "separation",
        glyph: "bludgeoning",
    },
    Formula {
        spell: "Incantation of Bludgeoning Vulnerability Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "bludgeoning",
    },
    Formula {
        spell: "Incantation of Brittlemail",
        quill: "infliction",
        ink: "objectification",
        glyph: "armor",
    },
    Formula {
        spell: "Incantation of Clumsiness Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "coordination",
    },
    Formula {
        spell: "Incantation of Cold Protection Self",
        quill: "introspection",
        ink: "formation",
        glyph: "frost",
    },
    Formula {
        spell: "Incantation of Cold Vulnerability Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "frost",
    },
    Formula {
        spell: "Incantation of Cooking Ineptitude Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "cooking",
    },
    Formula {
        spell: "Incantation of Cooking Mastery Self",
        quill: "introspection",
        ink: "formation",
        glyph: "cooking",
    },
    Formula {
        spell: "Incantation of Coordination Self",
        quill: "introspection",
        ink: "formation",
        glyph: "coordination",
    },
    Formula {
        spell: "Incantation of Corrosion",
        quill: "extraction",
        ink: "direction",
        glyph: "nether",
    },
    Formula {
        spell: "Incantation of Corruption",
        quill: "extraction",
        ink: "partition",
        glyph: "nether",
    },
    Formula {
        spell: "Incantation of Creature Enchantment Ineptitude Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "creature enchantment",
    },
    Formula {
        spell: "Incantation of Creature Enchantment Mastery Self",
        quill: "introspection",
        ink: "formation",
        glyph: "creature enchantment",
    },
    Formula {
        spell: "Incantation of Deception Ineptitude Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "deception",
    },
    Formula {
        spell: "Incantation of Deception Mastery Self",
        quill: "introspection",
        ink: "formation",
        glyph: "deception",
    },
    Formula {
        spell: "Incantation of Defenselessness Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "missile defense",
    },
    Formula {
        spell: "Incantation of Destructive Curse",
        quill: "extraction",
        ink: "conveyance",
        glyph: "nether",
    },
    Formula {
        spell: "Incantation of Dirty Fighting Ineptitude Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "dirty fighting",
    },
    Formula {
        spell: "Incantation of Dirty Fighting Mastery Self",
        quill: "introspection",
        ink: "formation",
        glyph: "dirty fighting",
    },
    Formula {
        spell: "Incantation of Drain Health Other",
        quill: "extraction",
        ink: "conveyance",
        glyph: "health",
    },
    Formula {
        spell: "Incantation of Drain Mana Other",
        quill: "extraction",
        ink: "conveyance",
        glyph: "mana",
    },
    Formula {
        spell: "Incantation of Drain Stamina Other",
        quill: "extraction",
        ink: "conveyance",
        glyph: "stamina",
    },
    Formula {
        spell: "Incantation of Dual Wield Ineptitude Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "dual wield",
    },
    Formula {
        spell: "Incantation of Dual Wield Mastery Self",
        quill: "introspection",
        ink: "formation",
        glyph: "dual wield",
    },
    Formula {
        spell: "Incantation of Endurance Self",
        quill: "introspection",
        ink: "formation",
        glyph: "endurance",
    },
    Formula {
        spell: "Incantation of Exhaustion Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "stamina regeneration",
    },
    Formula {
        spell: "Incantation of Faithlessness Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "loyalty",
    },
    Formula {
        spell: "Incantation of Fealty Self",
        quill: "introspection",
        ink: "formation",
        glyph: "loyalty",
    },
    Formula {
        spell: "Incantation of Feeblemind Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "self",
    },
    Formula {
        spell: "Incantation of Fester Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "health regeneration",
    },
    Formula {
        spell: "Incantation of Festering Curse",
        quill: "extraction",
        ink: "nullification",
        glyph: "healing",
    },
    Formula {
        spell: "Incantation of Finesse Weapon Ineptitude Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "finesse weapons",
    },
    Formula {
        spell: "Incantation of Finesse Weapon Mastery Self",
        quill: "introspection",
        ink: "formation",
        glyph: "finesse weapons",
    },
    Formula {
        spell: "Incantation of Fire Protection Self",
        quill: "introspection",
        ink: "formation",
        glyph: "flame",
    },
    Formula {
        spell: "Incantation of Fire Vulnerability Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "flame",
    },
    Formula {
        spell: "Incantation of Flame Arc",
        quill: "infliction",
        ink: "parabolic",
        glyph: "flame",
    },
    Formula {
        spell: "Incantation of Flame Bane",
        quill: "benevolence",
        ink: "objectification",
        glyph: "flame",
    },
    Formula {
        spell: "Incantation of Flame Blast",
        quill: "infliction",
        ink: "partition",
        glyph: "flame",
    },
    Formula {
        spell: "Incantation of Flame Bolt",
        quill: "infliction",
        ink: "direction",
        glyph: "flame",
    },
    Formula {
        spell: "Incantation of Flame Lure",
        quill: "infliction",
        ink: "objectification",
        glyph: "flame",
    },
    Formula {
        spell: "Incantation of Flame Streak",
        quill: "infliction",
        ink: "alacritous",
        glyph: "flame",
    },
    Formula {
        spell: "Incantation of Flame Volley",
        quill: "infliction",
        ink: "separation",
        glyph: "flame",
    },
    Formula {
        spell: "Incantation of Fletching Ineptitude Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "fletching",
    },
    Formula {
        spell: "Incantation of Fletching Mastery Self",
        quill: "introspection",
        ink: "formation",
        glyph: "fletching",
    },
    Formula {
        spell: "Incantation of Focus Self",
        quill: "introspection",
        ink: "formation",
        glyph: "focus",
    },
    Formula {
        spell: "Incantation of Force Arc",
        quill: "infliction",
        ink: "parabolic",
        glyph: "piercing",
    },
    Formula {
        spell: "Incantation of Force Blast",
        quill: "infliction",
        ink: "partition",
        glyph: "piercing",
    },
    Formula {
        spell: "Incantation of Force Bolt",
        quill: "infliction",
        ink: "direction",
        glyph: "piercing",
    },
    Formula {
        spell: "Incantation of Force Streak",
        quill: "infliction",
        ink: "alacritous",
        glyph: "piercing",
    },
    Formula {
        spell: "Incantation of Force Volley",
        quill: "infliction",
        ink: "separation",
        glyph: "piercing",
    },
    Formula {
        spell: "Incantation of Frailty Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "endurance",
    },
    Formula {
        spell: "Incantation of Frost Arc",
        quill: "infliction",
        ink: "parabolic",
        glyph: "frost",
    },
    Formula {
        spell: "Incantation of Frost Bane",
        quill: "benevolence",
        ink: "objectification",
        glyph: "frost",
    },
    Formula {
        spell: "Incantation of Frost Blast",
        quill: "infliction",
        ink: "partition",
        glyph: "frost",
    },
    Formula {
        spell: "Incantation of Frost Bolt",
        quill: "infliction",
        ink: "direction",
        glyph: "frost",
    },
    Formula {
        spell: "Incantation of Frost Lure",
        quill: "infliction",
        ink: "objectification",
        glyph: "frost",
    },
    Formula {
        spell: "Incantation of Frost Streak",
        quill: "infliction",
        ink: "alacritous",
        glyph: "frost",
    },
    Formula {
        spell: "Incantation of Frost Volley",
        quill: "infliction",
        ink: "separation",
        glyph: "frost",
    },
    Formula {
        spell: "Incantation of Harm Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "health",
    },
    Formula {
        spell: "Incantation of Heal Other",
        quill: "benevolence",
        ink: "conveyance",
        glyph: "health",
    },
    Formula {
        spell: "Incantation of Heal Self",
        quill: "introspection",
        ink: "formation",
        glyph: "health",
    },
    Formula {
        spell: "Incantation of Healing Ineptitude Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "healing",
    },
    Formula {
        spell: "Incantation of Healing Mastery Self",
        quill: "introspection",
        ink: "formation",
        glyph: "healing",
    },
    Formula {
        spell: "Incantation of Heavy Weapon Ineptitude Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "heavy weapons",
    },
    Formula {
        spell: "Incantation of Heavy Weapon Mastery Self",
        quill: "introspection",
        ink: "formation",
        glyph: "heavy weapons",
    },
    Formula {
        spell: "Incantation of Hermetic Void",
        quill: "infliction",
        ink: "objectification",
        glyph: "mana conversion",
    },
    Formula {
        spell: "Incantation of Impenetrability",
        quill: "benevolence",
        ink: "objectification",
        glyph: "armor",
    },
    Formula {
        spell: "Incantation of Impregnability Self",
        quill: "introspection",
        ink: "formation",
        glyph: "missile defense",
    },
    Formula {
        spell: "Incantation of Invulnerability Self",
        quill: "introspection",
        ink: "formation",
        glyph: "melee defense",
    },
    Formula {
        spell: "Incantation of Item Enchantment Ineptitude Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "item enchantment",
    },
    Formula {
        spell: "Incantation of Item Enchantment Mastery Self",
        quill: "introspection",
        ink: "formation",
        glyph: "item enchantment",
    },
    Formula {
        spell: "Incantation of Item Tinkering Expertise Self",
        quill: "introspection",
        ink: "formation",
        glyph: "item tinkering",
    },
    Formula {
        spell: "Incantation of Item Tinkering Ignorance Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "item tinkering",
    },
    Formula {
        spell: "Incantation of Jumping Ineptitude Self",
        quill: "infliction",
        ink: "conveyance",
        glyph: "jump",
    },
    Formula {
        spell: "Incantation of Jumping Mastery Self",
        quill: "introspection",
        ink: "formation",
        glyph: "jump",
    },
    Formula {
        spell: "Incantation of Leaden Feet Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "run",
    },
    Formula {
        spell: "Incantation of Leaden Weapon",
        quill: "infliction",
        ink: "objectification",
        glyph: "quickness",
    },
    Formula {
        spell: "Incantation of Leadership Ineptitude Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "leadership",
    },
    Formula {
        spell: "Incantation of Leadership Mastery Self",
        quill: "introspection",
        ink: "formation",
        glyph: "leadership",
    },
    Formula {
        spell: "Incantation of Life Magic Ineptitude Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "life magic",
    },
    Formula {
        spell: "Incantation of Life Magic Mastery Self",
        quill: "introspection",
        ink: "formation",
        glyph: "life magic",
    },
    Formula {
        spell: "Incantation of Light Weapon Ineptitude Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "light weapons",
    },
    Formula {
        spell: "Incantation of Light Weapon Mastery Self",
        quill: "introspection",
        ink: "formation",
        glyph: "light weapons",
    },
    Formula {
        spell: "Incantation of Lightning Arc",
        quill: "infliction",
        ink: "parabolic",
        glyph: "lightning",
    },
    Formula {
        spell: "Incantation of Lightning Bane",
        quill: "benevolence",
        ink: "objectification",
        glyph: "lightning",
    },
    Formula {
        spell: "Incantation of Lightning Blast",
        quill: "infliction",
        ink: "partition",
        glyph: "lightning",
    },
    Formula {
        spell: "Incantation of Lightning Bolt",
        quill: "infliction",
        ink: "direction",
        glyph: "lightning",
    },
    Formula {
        spell: "Incantation of Lightning Lure",
        quill: "infliction",
        ink: "objectification",
        glyph: "lightning",
    },
    Formula {
        spell: "Incantation of Lightning Protection Self",
        quill: "introspection",
        ink: "formation",
        glyph: "lightning",
    },
    Formula {
        spell: "Incantation of Lightning Streak",
        quill: "infliction",
        ink: "alacritous",
        glyph: "lightning",
    },
    Formula {
        spell: "Incantation of Lightning Volley",
        quill: "infliction",
        ink: "separation",
        glyph: "lightning",
    },
    Formula {
        spell: "Incantation of Lightning Vulnerability Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "lightning",
    },
    Formula {
        spell: "Incantation of Lockpick Ineptitude Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "lockpick",
    },
    Formula {
        spell: "Incantation of Lockpick Mastery Self",
        quill: "introspection",
        ink: "formation",
        glyph: "lockpick",
    },
    Formula {
        spell: "Incantation of Lure Blade",
        quill: "infliction",
        ink: "objectification",
        glyph: "melee defense",
    },
    Formula {
        spell: "Incantation of Magic Item Tinkering Expertise Self",
        quill: "introspection",
        ink: "formation",
        glyph: "magic item tinkering",
    },
    Formula {
        spell: "Incantation of Magic Item Tinkering Ignorance Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "magic item tinkering",
    },
    Formula {
        spell: "Incantation of Magic Resistance Self",
        quill: "introspection",
        ink: "formation",
        glyph: "magic defense",
    },
    Formula {
        spell: "Incantation of Magic Yield Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "magic defense",
    },
    Formula {
        spell: "Incantation of Mana Conversion Ineptitude Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "mana conversion",
    },
    Formula {
        spell: "Incantation of Mana Conversion Mastery Self",
        quill: "introspection",
        ink: "formation",
        glyph: "mana conversion",
    },
    Formula {
        spell: "Incantation of Mana Depletion Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "mana regeneration",
    },
    Formula {
        spell: "Incantation of Mana Drain Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "mana",
    },
    Formula {
        spell: "Incantation of Mana Renewal Self",
        quill: "introspection",
        ink: "formation",
        glyph: "mana regeneration",
    },
    Formula {
        spell: "Incantation of Missile Weapon Ineptitude Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "missile weapons",
    },
    Formula {
        spell: "Incantation of Missile Weapon Mastery Self",
        quill: "introspection",
        ink: "formation",
        glyph: "missile weapons",
    },
    Formula {
        spell: "Incantation of Monster Attunement Self",
        quill: "introspection",
        ink: "formation",
        glyph: "monster appraisal",
    },
    Formula {
        spell: "Incantation of Monster Unfamiliarity Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "monster appraisal",
    },
    Formula {
        spell: "Incantation of Nether Arc",
        quill: "infliction",
        ink: "parabolic",
        glyph: "nether",
    },
    Formula {
        spell: "Incantation of Nether Blast",
        quill: "infliction",
        ink: "partition",
        glyph: "nether",
    },
    Formula {
        spell: "Incantation of Nether Bolt",
        quill: "infliction",
        ink: "direction",
        glyph: "nether",
    },
    Formula {
        spell: "Incantation of Nether Streak",
        quill: "infliction",
        ink: "alacritous",
        glyph: "nether",
    },
    Formula {
        spell: "Incantation of Nullify Creature Magic Other",
        quill: "benevolence",
        ink: "nullification",
        glyph: "creature enchantment",
    },
    Formula {
        spell: "Incantation of Nullify Creature Magic Self",
        quill: "introspection",
        ink: "nullification",
        glyph: "creature enchantment",
    },
    Formula {
        spell: "Incantation of Nullify Item Magic",
        quill: "introspection",
        ink: "nullification",
        glyph: "item enchantment",
    },
    Formula {
        spell: "Incantation of Nullify Life Magic Other",
        quill: "benevolence",
        ink: "nullification",
        glyph: "life magic",
    },
    Formula {
        spell: "Incantation of Nullify Life Magic Self",
        quill: "introspection",
        ink: "nullification",
        glyph: "life magic",
    },
    Formula {
        spell: "Incantation of Person Attunement Self",
        quill: "introspection",
        ink: "formation",
        glyph: "person appraisal",
    },
    Formula {
        spell: "Incantation of Person Unfamiliarity Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "person appraisal",
    },
    Formula {
        spell: "Incantation of Piercing Bane",
        quill: "benevolence",
        ink: "objectification",
        glyph: "piercing",
    },
    Formula {
        spell: "Incantation of Piercing Lure",
        quill: "infliction",
        ink: "objectification",
        glyph: "piercing",
    },
    Formula {
        spell: "Incantation of Piercing Protection Self",
        quill: "introspection",
        ink: "formation",
        glyph: "piercing",
    },
    Formula {
        spell: "Incantation of Piercing Vulnerability Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "piercing",
    },
    Formula {
        spell: "Incantation of Quickness Self",
        quill: "introspection",
        ink: "formation",
        glyph: "quickness",
    },
    Formula {
        spell: "Incantation of Recklessness Ineptitude Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "recklessness",
    },
    Formula {
        spell: "Incantation of Recklessness Mastery Self",
        quill: "introspection",
        ink: "formation",
        glyph: "recklessness",
    },
    Formula {
        spell: "Incantation of Regeneration Self",
        quill: "introspection",
        ink: "formation",
        glyph: "regeneration",
    },
    Formula {
        spell: "Incantation of Rejuvenation Self",
        quill: "introspection",
        ink: "formation",
        glyph: "stamina regeneration",
    },
    Formula {
        spell: "Incantation of Revitalize Other",
        quill: "benevolence",
        ink: "conveyance",
        glyph: "stamina",
    },
    Formula {
        spell: "Incantation of Revitalize Self",
        quill: "introspection",
        ink: "formation",
        glyph: "stamina",
    },
    Formula {
        spell: "Incantation of Shield Ineptitude Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "shield",
    },
    Formula {
        spell: "Incantation of Shield Mastery Self",
        quill: "introspection",
        ink: "formation",
        glyph: "shield",
    },
    Formula {
        spell: "Incantation of Shock Arc",
        quill: "infliction",
        ink: "parabolic",
        glyph: "bludgeoning",
    },
    Formula {
        spell: "Incantation of Shock Blast",
        quill: "infliction",
        ink: "partition",
        glyph: "bludgeoning",
    },
    Formula {
        spell: "Incantation of Shock Wave",
        quill: "infliction",
        ink: "direction",
        glyph: "bludgeoning",
    },
    Formula {
        spell: "Incantation of Shock Wave Streak",
        quill: "infliction",
        ink: "alacritous",
        glyph: "bludgeoning",
    },
    Formula {
        spell: "Incantation of Slowness Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "quickness",
    },
    Formula {
        spell: "Incantation of Sneak Attack Ineptitude Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "sneak attack",
    },
    Formula {
        spell: "Incantation of Sneak Attack Mastery Self",
        quill: "introspection",
        ink: "formation",
        glyph: "sneak attack",
    },
    Formula {
        spell: "Incantation of Spirit Loather",
        quill: "infliction",
        ink: "objectification",
        glyph: "focus",
    },
    Formula {
        spell: "Incantation of Sprint Self",
        quill: "introspection",
        ink: "formation",
        glyph: "run",
    },
    Formula {
        spell: "Incantation of Strength Self",
        quill: "introspection",
        ink: "formation",
        glyph: "strength",
    },
    Formula {
        spell: "Incantation of Strengthen Lock",
        quill: "benevolence",
        ink: "objectification",
        glyph: "lockpick",
    },
    Formula {
        spell: "Incantation of Turn Blade",
        quill: "infliction",
        ink: "objectification",
        glyph: "coordination",
    },
    Formula {
        spell: "Incantation of Void Magic Ineptitude Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "void magic",
    },
    Formula {
        spell: "Incantation of Void Magic Mastery Self",
        quill: "introspection",
        ink: "formation",
        glyph: "void magic",
    },
    Formula {
        spell: "Incantation of Vulnerability Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "melee defense",
    },
    Formula {
        spell: "Incantation of War Magic Ineptitude Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "war magic",
    },
    Formula {
        spell: "Incantation of War Magic Mastery Self",
        quill: "introspection",
        ink: "formation",
        glyph: "war magic",
    },
    Formula {
        spell: "Incantation of Weaken Lock",
        quill: "infliction",
        ink: "objectification",
        glyph: "lockpick",
    },
    Formula {
        spell: "Incantation of Weakening Curse",
        quill: "extraction",
        ink: "nullification",
        glyph: "damage",
    },
    Formula {
        spell: "Incantation of Weakness Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "strength",
    },
    Formula {
        spell: "Incantation of Weapon Tinkering Expertise Self",
        quill: "introspection",
        ink: "formation",
        glyph: "weapon tinkering",
    },
    Formula {
        spell: "Incantation of Weapon Tinkering Ignorance Other",
        quill: "infliction",
        ink: "conveyance",
        glyph: "weapon tinkering",
    },
    Formula {
        spell: "Incantation of Whirling Blade",
        quill: "infliction",
        ink: "direction",
        glyph: "slashing",
    },
    Formula {
        spell: "Incantation of Whirling Blade Streak",
        quill: "infliction",
        ink: "alacritous",
        glyph: "slashing",
    },
    Formula {
        spell: "Incantation of Willpower Self",
        quill: "introspection",
        ink: "formation",
        glyph: "self",
    },
];

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
