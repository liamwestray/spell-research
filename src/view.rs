//! The window's markup and the rows it shows.
//!
//! One column per component role is what makes the request work: a component
//! the pack holds draws in full colour and one it lacks draws greyed, and
//! because the colour is per cell rather than per row, the three can differ on
//! the same line. The last column is a button per row, whose caption is empty
//! when the spell cannot be made — an empty caption draws no button, so a
//! spell you cannot craft simply has nothing to click.

use crate::craft::Job;
use crate::recipe::Recipe;
use crate::stock::Stock;

/// A row's text in full colour.
const COLOUR_HAVE: u32 = 0xFF_E8E8E8;
/// A row's text greyed: this component is missing.
const COLOUR_MISSING: u32 = 0xFF_5A5A5A;
/// The spell is already known.
const COLOUR_KNOWN: u32 = 0xFF_69B369;
/// A spell that can be made right now.
const COLOUR_READY: u32 = 0xFF_7EC8E3;

/// A binding's type.
///
/// The SDK's declaration and the markup checker each use an enum of this
/// shape, and there is no conversion between them, so the list is spelled
/// once here and mapped both ways. That is what keeps the window's bindings
/// and its markup from drifting apart.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Int,
    Text,
    Bool,
    UintList,
    TextList,
    /// A click.
    Action,
    /// A list row clicked.
    IntAction,
}

/// Every binding the markup names, and its type.
pub const BINDINGS: &[(&str, Kind)] = &[
    ("Summary", Kind::Text),
    ("Stock", Kind::Text),
    ("Selected", Kind::Int),
    ("Select", Kind::IntAction),
    ("Icons", Kind::UintList),
    ("Names", Kind::TextList),
    ("NameColours", Kind::UintList),
    ("QuillIcons", Kind::UintList),
    ("Quills", Kind::TextList),
    ("QuillColours", Kind::UintList),
    ("InkIcons", Kind::UintList),
    ("Inks", Kind::TextList),
    ("InkColours", Kind::UintList),
    ("GlyphIcons", Kind::UintList),
    ("Glyphs", Kind::TextList),
    ("GlyphColours", Kind::UintList),
    ("ScarabIcons", Kind::UintList),
    ("Have", Kind::TextList),
    ("HaveColours", Kind::UintList),
    ("Buttons", Kind::TextList),
    ("Craft", Kind::IntAction),
    ("Status", Kind::Text),
    ("Refresh", Kind::Action),
    ("CraftAll", Kind::Action),
    ("UnknownCaption", Kind::Text),
    ("ToggleUnknown", Kind::Action),
    ("ReadyCaption", Kind::Text),
    ("ToggleReady", Kind::Action),
    ("TabAll", Kind::Action),
    ("TabAllOn", Kind::Bool),
    ("TabCreature", Kind::Action),
    ("TabCreatureOn", Kind::Bool),
    ("TabItem", Kind::Action),
    ("TabItemOn", Kind::Bool),
    ("TabLife", Kind::Action),
    ("TabLifeOn", Kind::Bool),
    ("TabWarVoid", Kind::Action),
    ("TabWarVoidOn", Kind::Bool),
];

impl Kind {
    /// The SDK's spelling, for registering the panel.
    #[must_use]
    pub fn abi(self) -> openac_plugin::ui::BindingKind {
        use openac_plugin::ui::BindingKind as K;
        match self {
            Self::Int => K::Int,
            Self::Text => K::Text,
            Self::Bool => K::Bool,
            Self::UintList => K::UintList,
            Self::TextList => K::TextList,
            Self::Action => K::Action,
            Self::IntAction => K::IntAction,
        }
    }
}

/// The window.
pub const MARKUP: &str = r#"<panel w="700" h="480">
  <label x="10" y="8" text="{Summary}"/>
  <label x="10" y="26" text="{Stock}"/>
  <tab x="10" y="44" w="88" h="20" text="All" selected="{TabAllOn}" onclick="{TabAll}"/>
  <tab x="102" y="44" w="88" h="20" text="Creature" selected="{TabCreatureOn}" onclick="{TabCreature}"/>
  <tab x="194" y="44" w="88" h="20" text="Item" selected="{TabItemOn}" onclick="{TabItem}"/>
  <tab x="286" y="44" w="88" h="20" text="Life" selected="{TabLifeOn}" onclick="{TabLife}"/>
  <tab x="378" y="44" w="88" h="20" text="War/Void" selected="{TabWarVoidOn}" onclick="{TabWarVoid}"/>
    <list x="10" y="68" w="680" h="336" rowheight="24" selected="{Selected}" onchange="{Select}">
    <column type="icon" iconkind="spell" values="{Icons}" onclick="{Select}" width="30"/>
    <column type="text" items="{Names}" colors="{NameColours}" header="Spell" sort="text" width="190"/>
    <column type="icon" iconkind="did" values="{QuillIcons}" onclick="{Select}" width="26"/>
    <column type="text" items="{Quills}" colors="{QuillColours}" header="Quill" width="84"/>
    <column type="icon" iconkind="did" values="{InkIcons}" onclick="{Select}" width="26"/>
    <column type="text" items="{Inks}" colors="{InkColours}" header="Ink" width="84"/>
    <column type="icon" iconkind="did" values="{GlyphIcons}" onclick="{Select}" width="26"/>
    <column type="text" items="{Glyphs}" colors="{GlyphColours}" header="Glyph" width="96"/>
    <column type="icon" iconkind="did" values="{ScarabIcons}" onclick="{Select}" width="26"/>
    <column type="text" items="{Have}" colors="{HaveColours}" header="Have" sort="number" width="42"/>
    <column type="button" items="{Buttons}" onclick="{Craft}" header="" buttonw="62"/>
  </list>
  <label x="10" y="410" w="680" h="18" text="{Status}"/>
  <button x="10" y="434" w="84" h="24" text="Refresh" onclick="{Refresh}"/>
  <button x="100" y="434" w="90" h="24" text="Craft all" onclick="{CraftAll}"/>
  <button x="196" y="434" w="104" h="24" text="{UnknownCaption}" onclick="{ToggleUnknown}"/>
  <button x="306" y="434" w="104" h="24" text="{ReadyCaption}" onclick="{ToggleReady}"/>
</panel>"#;

/// Which school tab is showing. "Craft all" only ever queues the rows the
/// tab shows, so a tab is also the answer to "craft this school, not
/// everything".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tab {
    #[default]
    All,
    Creature,
    Item,
    Life,
    /// War and void share a tab: both are the mage damage schools, and
    /// neither is worth a full tab alone.
    WarVoid,
}

/// The magic skill a school is cast with, as `SpellInfo::school` spells it.
const CREATURE: u32 = 31;
const ITEM: u32 = 32;
const LIFE: u32 = 33;
const WAR: u32 = 34;
const VOID: u32 = 43;

impl Tab {
    /// Whether a spell cast with `school` shows on this tab. A school the
    /// plugin does not recognise shows only under All, never hidden.
    #[must_use]
    pub fn shows(self, school: u32) -> bool {
        match self {
            Self::All => true,
            Self::Creature => school == CREATURE,
            Self::Item => school == ITEM,
            Self::Life => school == LIFE,
            Self::WarVoid => school == WAR || school == VOID,
        }
    }

    /// Its caption, for the header.
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::All => "All",
            Self::Creature => "Creature",
            Self::Item => "Item",
            Self::Life => "Life",
            Self::WarVoid => "War/Void",
        }
    }
}

/// What the rows hold, one entry per list column.
#[derive(Debug, Default, Clone)]
pub struct Rows {
    /// The spell of each row, so a row index finds its recipe.
    pub spell_ids: Vec<u32>,
    /// Each row's school, so the tab filter can reach it by row index.
    pub schools: Vec<u32>,
    pub icons: Vec<u32>,
    pub names: Vec<String>,
    pub name_colours: Vec<u32>,
    /// One icon column per component: the held item's real icon, or 0 —
    /// which draws nothing — when the pack lacks one. The name beside it
    /// greys the same way.
    pub quill_icons: Vec<u32>,
    pub quills: Vec<String>,
    pub quill_colours: Vec<u32>,
    pub ink_icons: Vec<u32>,
    pub inks: Vec<String>,
    pub ink_colours: Vec<u32>,
    pub glyph_icons: Vec<u32>,
    pub glyphs: Vec<String>,
    pub glyph_colours: Vec<u32>,
    pub scarab_icons: Vec<u32>,
    pub have: Vec<String>,
    pub have_colours: Vec<u32>,
    /// A row's button caption; empty draws no button.
    pub buttons: Vec<String>,
}

impl Rows {
    /// The rows for `recipes`, given what is held and what is known.
    #[must_use]
    pub fn build(recipes: &[Recipe], stock: &Stock, known: &dyn Fn(u32) -> bool) -> Self {
        let mut rows = Self::default();
        for recipe in recipes {
            let wants = recipe.requirements();
            // One cell per component: its icon surface (0 when the pack
            // lacks one or the client has not described it).
            let cell = |role: crate::component::Role| -> u32 {
                wants
                    .iter()
                    .find(|want| want.role == role)
                    .map_or(0, |want| stock.icon(want))
            };
            let quill_icon = cell(crate::component::Role::Quill);
            let ink_icon = cell(crate::component::Role::Ink);
            let glyph_icon = cell(crate::component::Role::Glyph);
            let scarab_icon = cell(crate::component::Role::Scarab);
            let held = usize::from(quill_icon != 0)
                + usize::from(ink_icon != 0)
                + usize::from(glyph_icon != 0)
                + usize::from(scarab_icon != 0);
            let total = wants.len();

            let is_known = known(recipe.spell_id);
            let craftable = !is_known && held == total;

            rows.spell_ids.push(recipe.spell_id);
            rows.schools.push(recipe.school);
            rows.icons.push(recipe.icon_id);
            rows.names.push(recipe.name.clone());
            rows.name_colours.push(if is_known {
                COLOUR_KNOWN
            } else if craftable {
                COLOUR_READY
            } else {
                COLOUR_HAVE
            });
            rows.quill_icons.push(quill_icon);
            rows.quills.push(crate::component::title(recipe.quill));
            rows.quill_colours.push(if quill_icon != 0 {
                COLOUR_HAVE
            } else {
                COLOUR_MISSING
            });
            rows.ink_icons.push(ink_icon);
            rows.inks.push(crate::component::title(recipe.ink));
            rows.ink_colours.push(if ink_icon != 0 {
                COLOUR_HAVE
            } else {
                COLOUR_MISSING
            });
            rows.glyph_icons.push(glyph_icon);
            rows.glyphs.push(crate::component::title(recipe.glyph));
            rows.glyph_colours.push(if glyph_icon != 0 {
                COLOUR_HAVE
            } else {
                COLOUR_MISSING
            });
            rows.scarab_icons.push(scarab_icon);
            rows.have.push(format!("{held}/{total}"));
            rows.have_colours.push(if craftable {
                COLOUR_READY
            } else if held == 0 {
                COLOUR_MISSING
            } else {
                COLOUR_HAVE
            });
            rows.buttons.push(if craftable {
                "Craft".to_owned()
            } else {
                String::new()
            });
        }
        rows
    }

    /// The rows with a button left, for "craft all".
    #[must_use]
    pub fn ready(&self) -> Vec<u32> {
        self.spell_ids
            .iter()
            .zip(&self.buttons)
            .filter(|(_, caption)| !caption.is_empty())
            .map(|(id, _)| *id)
            .collect()
    }

    /// The rows `keep` accepts, every column moving together so a row's cells
    /// stay on one line.
    #[must_use]
    pub fn filtered(&self, keep: impl Fn(usize) -> bool) -> Self {
        let take = |column: &Vec<String>| {
            column
                .iter()
                .enumerate()
                .filter(|(index, _)| keep(*index))
                .map(|(_, value)| value.clone())
                .collect()
        };
        let numbers = |column: &Vec<u32>| {
            column
                .iter()
                .enumerate()
                .filter(|(index, _)| keep(*index))
                .map(|(_, value)| *value)
                .collect()
        };
        Self {
            spell_ids: numbers(&self.spell_ids),
            schools: numbers(&self.schools),
            icons: numbers(&self.icons),
            names: take(&self.names),
            name_colours: numbers(&self.name_colours),
            quill_icons: numbers(&self.quill_icons),
            quills: take(&self.quills),
            quill_colours: numbers(&self.quill_colours),
            ink_icons: numbers(&self.ink_icons),
            inks: take(&self.inks),
            ink_colours: numbers(&self.ink_colours),
            glyph_icons: numbers(&self.glyph_icons),
            glyphs: take(&self.glyphs),
            glyph_colours: numbers(&self.glyph_colours),
            scarab_icons: numbers(&self.scarab_icons),
            have: take(&self.have),
            have_colours: numbers(&self.have_colours),
            buttons: take(&self.buttons),
        }
    }

    /// One row per column: the UI cannot be handed rows whose columns disagree.
    #[must_use]
    pub fn is_square(&self) -> bool {
        let height = self.spell_ids.len();
        [
            self.schools.len(),
            self.icons.len(),
            self.names.len(),
            self.name_colours.len(),
            self.quill_icons.len(),
            self.quills.len(),
            self.quill_colours.len(),
            self.ink_icons.len(),
            self.inks.len(),
            self.ink_colours.len(),
            self.glyph_icons.len(),
            self.glyphs.len(),
            self.glyph_colours.len(),
            self.scarab_icons.len(),
            self.have.len(),
            self.have_colours.len(),
            self.buttons.len(),
        ]
        .iter()
        .all(|len| *len == height)
    }
}

/// The header line: what the tab holds, and how much of it is ready.
#[must_use]
pub fn summary(tab: Tab, total: usize, known: usize, ready: usize) -> String {
    format!(
        "Level VIII [{}]: {total} researchable, {known} known, {} to research, {ready} ready to craft.",
        tab.label(),
        total.saturating_sub(known)
    )
}

/// The pack line: what each component family holds.
#[must_use]
pub fn stock_line(stock: &Stock) -> String {
    let totals = stock.totals();
    if totals.is_empty() {
        return "Pack: no spell components.".to_owned();
    }
    let parts: Vec<String> = totals
        .iter()
        .map(|(name, count)| format!("{name} {count}"))
        .collect();
    format!("Pack: {}", parts.join(", "))
}

/// The job a row starts: the quill in hand, the other three to combine.
#[must_use]
pub fn job_for(recipe: &Recipe, stock: &Stock) -> Option<Job> {
    let quill = recipe
        .requirements()
        .into_iter()
        .find(|want| want.role == crate::component::Role::Quill)?;
    let held = stock.find(&quill)?;
    Some(Job::new(
        recipe.spell_id,
        recipe.name.clone(),
        held.object_id,
        crate::craft::steps_for(recipe),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use openac_plugin::items::InventoryItem;

    /// A spell wanting the quill, ink and glyph variants named.
    fn recipe(quill: &'static str, ink: &'static str, glyph: &'static str) -> Recipe {
        recipe_with(1, 33, quill, ink, glyph)
    }

    /// … with its id and school chosen too.
    fn recipe_with(
        spell_id: u32,
        school: u32,
        quill: &'static str,
        ink: &'static str,
        glyph: &'static str,
    ) -> Recipe {
        Recipe {
            spell_id,
            ids: vec![spell_id],
            name: "Test Spell".to_owned(),
            school,
            icon_id: 9,
            quill,
            ink,
            glyph,
        }
    }

    fn item(object_id: u32, name: &str) -> InventoryItem {
        InventoryItem {
            object_id,
            name: name.to_owned(),
            stack_size: 1,
            ..Default::default()
        }
    }

    fn stock_of(names: &[&str]) -> Stock {
        let mut stock = Stock::from_items(
            &names
                .iter()
                .enumerate()
                .map(|(i, name)| item(i as u32 + 1, name))
                .collect::<Vec<_>>(),
        );
        // The real client describes every item in the pack, so every held
        // component has an icon; tests give each one a distinct surface.
        stock.fill_icons(|object| 0x0600_0000 + object);
        stock
    }

    /// The three component colours of row 0.
    fn colours(rows: &Rows) -> [u32; 3] {
        [
            rows.quill_colours[0],
            rows.ink_colours[0],
            rows.glyph_colours[0],
        ]
    }

    #[test]
    fn a_component_the_pack_lacks_greys_and_one_it_holds_does_not() {
        let recipes = vec![recipe("benevolence", "conveyance", "strength")];
        // The quill and the ink are held; the glyph is not.
        let rows = Rows::build(
            &recipes,
            &stock_of(&["Quill of Benevolence", "Ink of Conveyance"]),
            &|_| false,
        );
        assert_eq!(colours(&rows), [COLOUR_HAVE, COLOUR_HAVE, COLOUR_MISSING]);
        assert_eq!(rows.glyphs[0], "Strength", "the name still shows");
        assert_eq!(rows.have[0], "2/4", "the scarab is wanted too");
    }

    #[test]
    fn the_right_ink_is_not_mistaken_for_the_one_the_spell_wants() {
        let recipes = vec![recipe("benevolence", "conveyance", "strength")];
        let rows = Rows::build(
            &recipes,
            &stock_of(&[
                "Ink of Direction",
                "Quill of Benevolence",
                "Glyph of Strength",
            ]),
            &|_| false,
        );
        assert_eq!(rows.ink_colours[0], COLOUR_MISSING);
        assert_eq!(rows.have[0], "2/4");
    }

    #[test]
    fn a_spell_with_everything_in_hand_gets_a_button() {
        let recipes = vec![recipe("benevolence", "conveyance", "strength")];
        let rows = Rows::build(
            &recipes,
            &stock_of(&[
                "Quill of Benevolence",
                "Mana Scarab",
                "Ink of Conveyance",
                "Glyph of Strength",
            ]),
            &|_| false,
        );
        assert_eq!(rows.buttons[0], "Craft");
        assert_eq!(rows.have[0], "4/4");
        assert_eq!(rows.ready(), vec![1]);
    }

    #[test]
    fn a_known_spell_offers_no_button() {
        let recipes = vec![recipe("benevolence", "conveyance", "strength")];
        let rows = Rows::build(
            &recipes,
            &stock_of(&[
                "Quill of Benevolence",
                "Mana Scarab",
                "Ink of Conveyance",
                "Glyph of Strength",
            ]),
            &|_| true,
        );
        assert!(rows.buttons[0].is_empty(), "nothing to click");
        assert_eq!(rows.name_colours[0], COLOUR_KNOWN);
        assert!(rows.ready().is_empty());
    }

    #[test]
    fn no_scarab_means_no_button() {
        let recipes = vec![recipe("benevolence", "conveyance", "strength")];
        let rows = Rows::build(
            &recipes,
            &stock_of(&[
                "Quill of Benevolence",
                "Ink of Conveyance",
                "Glyph of Strength",
            ]),
            &|_| false,
        );
        assert!(
            rows.buttons[0].is_empty(),
            "the infusion has nothing to burn"
        );
        assert_eq!(rows.have[0], "3/4");
    }

    #[test]
    fn a_learning_cost_scarab_is_not_a_mana_scarab() {
        let recipes = vec![recipe("benevolence", "conveyance", "strength")];
        let rows = Rows::build(
            &recipes,
            &stock_of(&[
                "Quill of Benevolence",
                "Copper Scarab",
                "Ink of Conveyance",
                "Glyph of Strength",
            ]),
            &|_| false,
        );
        assert!(
            rows.buttons[0].is_empty(),
            "copper does not infuse anything"
        );
    }

    #[test]
    fn a_held_component_draws_its_icon_and_a_missing_one_draws_nothing() {
        let recipes = vec![recipe("benevolence", "conveyance", "strength")];
        let rows = Rows::build(
            &recipes,
            &stock_of(&["Quill of Benevolence", "Ink of Conveyance"]),
            &|_| false,
        );
        // stock_of gives every held item 0x0600_0000 + its object id.
        assert_eq!(rows.quill_icons[0], 0x0600_0001);
        assert_eq!(rows.ink_icons[0], 0x0600_0002);
        assert_eq!(rows.glyph_icons[0], 0, "the glyph is not in the pack");
        assert_eq!(rows.scarab_icons[0], 0, "nor is the scarab");
        assert_eq!(rows.quill_colours[0], COLOUR_HAVE);
        assert_eq!(rows.glyph_colours[0], COLOUR_MISSING);
    }

    #[test]
    fn filtering_keeps_every_column_the_same_length() {
        let recipes = vec![
            recipe("benevolence", "conveyance", "strength"),
            recipe("infliction", "parabolic", "flame"),
        ];
        let all = Rows::build(&recipes, &Stock::default(), &|_| false);
        assert_eq!(all.spell_ids.len(), 2);
        let only_second = all.filtered(|index| index == 1);
        assert!(only_second.is_square());
        assert_eq!(only_second.names.len(), 1);
        assert_eq!(only_second.quills[0], "Infliction");
        assert_eq!(only_second.glyphs[0], "Flame");
    }

    #[test]
    fn the_summary_counts_what_is_left_and_ready() {
        let text = summary(Tab::Life, 47, 12, 6);
        assert!(text.contains("[Life]"), "{text}");
        assert!(text.contains("47 researchable"), "{text}");
        assert!(text.contains("35 to research"), "{text}");
        assert!(text.contains("6 ready"), "{text}");
    }

    #[test]
    fn tabs_group_the_schools() {
        assert!(Tab::Creature.shows(31));
        assert!(Tab::Item.shows(32));
        assert!(Tab::Life.shows(33));
        assert!(Tab::WarVoid.shows(34), "war");
        assert!(Tab::WarVoid.shows(43), "void");
        assert!(!Tab::WarVoid.shows(33));
        // Every school shows somewhere, and an unknown one shows under All.
        for school in [31, 32, 33, 34, 43, 0] {
            assert!(Tab::All.shows(school), "{school}");
            assert!(
                [Tab::Creature, Tab::Item, Tab::Life, Tab::WarVoid]
                    .iter()
                    .any(|t| t.shows(school))
                    || school == 0,
                "{school} has no tab"
            );
        }
    }

    #[test]
    fn a_tab_filter_keeps_only_its_school_and_craft_all_follows_it() {
        let recipes = vec![
            recipe_with(1, 31, "benevolence", "conveyance", "strength"),
            recipe_with(2, 34, "infliction", "parabolic", "flame"),
            recipe_with(3, 33, "introspection", "formation", "health"),
        ];
        let stock = stock_of(&[
            "Quill of Benevolence",
            "Mana Scarab",
            "Ink of Conveyance",
            "Glyph of Strength",
            "Quill of Infliction",
            "Parabolic Ink",
            "Glyph of Flame",
            "Quill of Introspection",
            "Ink of Formation",
            "Glyph of Health",
        ]);
        let all = Rows::build(&recipes, &stock, &|_| false);
        let creature = all.filtered(|index| Tab::Creature.shows(all.schools[index]));
        assert_eq!(creature.spell_ids, vec![1]);
        // "Craft all" reads the shown rows, so it queues the tab only.
        assert_eq!(creature.ready(), vec![1]);
        let war = all.filtered(|index| Tab::WarVoid.shows(all.schools[index]));
        assert_eq!(war.ready(), vec![2]);
        assert_eq!(all.ready(), vec![1, 2, 3], "All still means all");
    }

    #[test]
    fn a_job_starts_from_the_quill_and_has_three_combines_left() {
        let r = recipe("benevolence", "conveyance", "strength");
        let stock = stock_of(&["Quill of Benevolence", "Mana Scarab"]);
        let job = job_for(&r, &stock).expect("a quill is held");
        assert_eq!(job.remaining(), 3, "scarab, ink and glyph");
    }

    #[test]
    fn no_quill_means_no_job() {
        let r = recipe("benevolence", "conveyance", "strength");
        assert!(job_for(&r, &stock_of(&["Mana Scarab"])).is_none());
    }

    /// The window is a string, and a misspelled attribute means the player
    /// simply never sees it — so the markup is parsed here, against the same
    /// binding list the plugin registers, rather than at first run.
    #[test]
    fn the_window_markup_parses_with_the_declared_bindings() {
        use ac_plugin_ui::BindingKind as K;
        use ac_plugin_ui::BindingSchema;

        let ui_kind = |kind: Kind| match kind {
            Kind::Int => K::Int,
            Kind::Text => K::Text,
            Kind::Bool => K::Bool,
            Kind::UintList => K::UintList,
            Kind::TextList => K::TextList,
            Kind::Action => K::Action,
            Kind::IntAction => K::IntAction,
        };

        let mut schema = BindingSchema::new("SpellResearch");
        for (name, kind) in BINDINGS {
            schema.declare(*name, ui_kind(*kind));
        }
        if let Err(error) = ac_plugin_ui::markup::parse(MARKUP, &schema) {
            panic!("the window's markup does not parse: {}", error.0);
        }
    }

    #[test]
    fn every_binding_is_declared_and_used() {
        // A binding in the list that the markup never names is dead weight;
        // one the markup names that the list omits fails to parse, which the
        // test above already catches.
        for (name, _) in BINDINGS {
            assert!(
                MARKUP.contains(&format!("{{{name}}}")) || MARKUP.contains(name),
                "{name} is declared but the markup never uses it"
            );
        }
    }
}
