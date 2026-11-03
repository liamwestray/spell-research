//! Spell Research — tracks and automates level VIII spell scroll crafting.
//!
//! The window lists the level VIII incantations this plugin's recipe table
//! knows — and this server has — with their three components greyed when
//! the pack lacks them, and a Craft button on the ones that can be made
//! now.
//!
//! The recipes are not read from the game: the spell table's formula
//! components are the old reagent costs for *learning* a spell, not the
//! quill, ink and glyph that *craft* its scroll, and its spell "level" is
//! a client heuristic. [`formulas`] carries the community's record of the
//! real recipes, transcribed from the wiki; the spell table is used only
//! to look each spell up by name for its id, icon and school.
//! [`component::classify`] accepts only the materials the same page names,
//! so a taper or a copper scarab in the pack counts toward nothing.
//!
//! `/spellresearch` reports in chat (add `full` for one line per spell);
//! `/spellresearch show` opens the window.

// `export_plugin!` wires the entry point only for wasm32, so a host build
// (which exists to run the unit tests) has nothing that reaches the plugin's
// own code and reports all of it dead.
#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]

mod component;
mod craft;
mod formulas;
mod recipe;
mod stock;
mod view;

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::craft::{Action, Job, Snapshot};
use crate::recipe::Recipe;
use crate::stock::Stock;
use crate::view::{Rows, Tab};
use openac_plugin::ui::{ActionValue, Panel, PanelSpec, TypedValue, UiEvent};
use openac_plugin::{Event, Host, Plugin, StartInfo, items::ItemStatus};

/// Ticks between re-reads while idle (a tick is 15 ms).
const IDLE_TICKS: u32 = 100;
/// … and while a craft is running, so the next combine starts promptly.
const BUSY_TICKS: u32 = 4;

#[derive(Default)]
struct SpellResearch {
    panel: Option<Panel>,
    /// Every researchable spell this server has, in name order.
    recipes: Vec<Recipe>,
    /// The rows currently shown, which the UI's row indexes refer to.
    rows: Rows,
    /// Spells asked for and not started yet.
    queue: VecDeque<u32>,
    /// The job running now.
    active: Option<Job>,
    ticks: u32,
    /// The spells on screen right now. The queue and the running job only
    /// ever touch these: switch tab or toggle a filter and anything no
    /// longer visible stops being crafted.
    visible: BTreeSet<u32>,
    /// The school tab showing. Scopes the list, the header and "Craft all".
    tab: Tab,
    unknown_only: bool,
    ready_only: bool,
    status: String,
    built: bool,
}

impl Plugin for SpellResearch {
    fn enable(&mut self, host: &mut Host<'_>, _info: &StartInfo) -> Result<(), String> {
        host.register_command(
            "spellresearch",
            "Level VIII spell research: [show] opens the window, [full] lists every spell",
        )?;
        let spec = view::BINDINGS.iter().fold(
            PanelSpec::new("main", "Spell Research", view::MARKUP),
            |spec, (name, kind)| spec.bind(name, kind.abi()),
        );
        self.panel = Some(host.register_panel(&spec)?);
        Ok(())
    }

    fn tick(&mut self, host: &mut Host<'_>) {
        if !host.item_state().is_available {
            return;
        }
        self.ticks = self.ticks.saturating_add(1);
        let cadence = if self.active.is_some() || !self.queue.is_empty() {
            BUSY_TICKS
        } else {
            IDLE_TICKS
        };
        if self.ticks < cadence {
            return;
        }
        self.ticks = 0;
        if !self.built {
            self.build(host);
        }
        self.pump(host);
        self.refresh(host);
    }

    fn on_event(&mut self, host: &mut Host<'_>, event: Event) {
        match event {
            Event::Command(call) if call.verb == "spellresearch" => {
                self.handle_command(host, call.arguments.trim());
            }
            Event::Ui(UiEvent::Action(action)) => {
                self.handle_ui(host, &action.handler, action.value);
            }
            _ => {}
        }
    }
}

impl SpellResearch {
    /// Match the recipe table against the spell table, by name.
    fn build(&mut self, host: &mut Host<'_>) {
        let spells = host.all_spells();
        // The server can list one spell name under several ids; the recipe
        // keeps them all so knowing any of them counts as knowing the spell.
        let mut by_name: BTreeMap<&str, Vec<&openac_plugin::magic::SpellInfo>> = BTreeMap::new();
        for spell in &spells {
            by_name.entry(spell.name.as_str()).or_default().push(spell);
        }

        let mut recipes = Vec::with_capacity(formulas::FORMULAS.len());
        let mut absent = 0;
        for formula in formulas::FORMULAS {
            match by_name.get(formula.spell) {
                Some(entries) => {
                    if let Some(recipe) = Recipe::new(formula, entries.clone()) {
                        recipes.push(recipe);
                    }
                }
                // The wiki knows a recipe this server has no spell for;
                // there is nothing to research, so nothing to show.
                None => absent += 1,
            }
        }
        self.status = if absent == 0 {
            format!(
                "Read {} recipes from the spell research table.",
                recipes.len()
            )
        } else {
            format!(
                "Read {} recipes; {absent} have no spell on this server.",
                recipes.len()
            )
        };
        self.recipes = recipes;
        self.built = true;
    }

    /// Rebuild the rows and push them to the window.
    fn refresh(&mut self, host: &mut Host<'_>) {
        let Some(panel) = self.panel.as_mut() else {
            return;
        };
        let mut stock = Stock::from_items(&host.owned_items());
        // Icons live on the world's object records, not the inventory list.
        stock.fill_icons(|object| host.object(object).map_or(0, |o| o.icon_id));
        // A spell is known if the book holds any of the ids the table lists
        // under its name — the server carries some spells more than once,
        // and which one a character learned is not the plugin's choice.
        // Ask per id (the known lists are filtered three ways and none
        // holds every spell), then fold the answers back onto the rows.
        let ids: Vec<u32> = self.recipes.iter().flat_map(|r| r.ids.clone()).collect();
        let book: BTreeSet<u32> = ids
            .into_iter()
            .filter(|id| host.is_spell_known(*id))
            .collect();
        let known: BTreeSet<u32> = self
            .recipes
            .iter()
            .filter(|recipe| recipe.is_known(&|id| book.contains(&id)))
            .map(|recipe| recipe.spell_id)
            .collect();
        let all = Rows::build(&self.recipes, &stock, &|id| known.contains(&id));
        // The tab scopes everything below it: the list, the header, and
        // "Craft all", which queues only the rows on show.
        let tabbed = all.filtered(|index| self.tab.shows(all.schools[index]));
        let tab_known = tabbed
            .spell_ids
            .iter()
            .filter(|id| known.contains(id))
            .count();
        let rows = tabbed.filtered(|index| {
            let id = tabbed.spell_ids[index];
            (!self.unknown_only || !known.contains(&id))
                && (!self.ready_only || !tabbed.buttons[index].is_empty())
        });

        let ready = rows.ready().len();
        panel.set(
            "Summary",
            view::summary(self.tab, tabbed.spell_ids.len(), tab_known, ready),
        );
        panel.set("Stock", view::stock_line(&stock));
        panel.set("Icons", TypedValue::UintList(rows.icons.clone()));
        panel.set("Names", TypedValue::TextList(rows.names.clone()));
        panel.set(
            "NameColours",
            TypedValue::UintList(rows.name_colours.clone()),
        );
        panel.set("QuillIcons", TypedValue::UintList(rows.quill_icons.clone()));
        panel.set("Quills", TypedValue::TextList(rows.quills.clone()));
        panel.set(
            "QuillColours",
            TypedValue::UintList(rows.quill_colours.clone()),
        );
        panel.set("InkIcons", TypedValue::UintList(rows.ink_icons.clone()));
        panel.set("Inks", TypedValue::TextList(rows.inks.clone()));
        panel.set("InkColours", TypedValue::UintList(rows.ink_colours.clone()));
        panel.set("GlyphIcons", TypedValue::UintList(rows.glyph_icons.clone()));
        panel.set("Glyphs", TypedValue::TextList(rows.glyphs.clone()));
        panel.set(
            "GlyphColours",
            TypedValue::UintList(rows.glyph_colours.clone()),
        );
        panel.set(
            "ScarabIcons",
            TypedValue::UintList(rows.scarab_icons.clone()),
        );
        panel.set("Have", TypedValue::TextList(rows.have.clone()));
        panel.set(
            "HaveColours",
            TypedValue::UintList(rows.have_colours.clone()),
        );
        panel.set("Buttons", TypedValue::TextList(rows.buttons.clone()));
        panel.set("Status", self.status.clone());
        panel.set(
            "UnknownCaption",
            if self.unknown_only {
                "Showing unknown"
            } else {
                "Show unknown only"
            },
        );
        panel.set(
            "ReadyCaption",
            if self.ready_only {
                "Showing ready"
            } else {
                "Show ready only"
            },
        );
        panel.set("TabAllOn", TypedValue::Bool(self.tab == Tab::All));
        panel.set("TabCreatureOn", TypedValue::Bool(self.tab == Tab::Creature));
        panel.set("TabItemOn", TypedValue::Bool(self.tab == Tab::Item));
        panel.set("TabLifeOn", TypedValue::Bool(self.tab == Tab::Life));
        panel.set("TabWarVoidOn", TypedValue::Bool(self.tab == Tab::WarVoid));
        debug_assert!(rows.is_square(), "the list's columns must stay aligned");
        self.visible = rows.spell_ids.iter().copied().collect();
        self.rows = rows;
        if let Err(why) = panel.flush(host) {
            self.status = format!("The window could not be updated: {why}");
        }
    }

    /// Start and advance the craft in progress.
    fn pump(&mut self, host: &mut Host<'_>) {
        let state = host.item_state();
        let items = host.owned_items();
        let stock = Stock::from_items(&items);
        let snapshot = Snapshot {
            objects: items.iter().map(|item| item.object_id).collect(),
            revision: state.last_completion.revision,
            error: state.last_completion.weenie_error,
        };

        // Nothing running: take the next spell asked for. Anything the
        // window no longer shows — tab switched, filter toggled, spell
        // learned — is dropped rather than crafted.
        if self.active.is_none() {
            while let Some(id) = self.queue.pop_front() {
                if !self.visible.contains(&id) {
                    continue;
                }
                let Some(recipe) = self.recipes.iter().find(|recipe| recipe.spell_id == id) else {
                    continue;
                };
                match view::job_for(recipe, &stock) {
                    Some(job) => {
                        self.status = format!(
                            "Crafting {} ({} combines left).",
                            recipe.name,
                            job.remaining()
                        );
                        self.active = Some(job);
                        break;
                    }
                    None => self.status = format!("{}: no quill in the pack.", recipe.name),
                }
            }
        }

        let Some(mut job) = self.active.take() else {
            return;
        };
        // The job's spell left the window. A combine already sent cannot be
        // recalled, so an in-flight job is left to settle; one that has not
        // been asked to combine anything is simply stopped.
        if !self.visible.contains(&job.spell_id) && !job.is_in_flight() {
            self.status = format!(
                "Stopped {}: not on the {} tab.",
                job.spell_name,
                self.tab.label()
            );
            self.queue.retain(|id| self.visible.contains(id));
            return;
        }
        // Another use is in flight; the pack will change under us.
        if state.is_busy {
            self.active = Some(job);
            return;
        }

        let action = job.drive(&snapshot, |want| {
            stock.find(want).map(|held| held.object_id)
        });
        // The in-flight combine has just settled and the spell is no longer
        // on screen: do not send another.
        if !self.visible.contains(&job.spell_id) && matches!(action, Action::Apply { .. }) {
            self.status = format!(
                "Stopped {}: not on the {} tab.",
                job.spell_name,
                self.tab.label()
            );
            self.queue.retain(|id| self.visible.contains(id));
            return;
        }
        match action {
            Action::Waiting => self.active = Some(job),
            Action::Apply { base, target } => {
                let result = host.apply_item(base, target);
                match result.status {
                    ItemStatus::Started | ItemStatus::Completed => self.active = Some(job),
                    // Nothing was consumed; ask again when the pack frees up.
                    ItemStatus::Busy => {
                        job.undo();
                        self.active = Some(job);
                    }
                    other => {
                        job.undo();
                        self.active = Some(job);
                        self.status = format!(
                            "Could not combine: {:?}{}",
                            other,
                            result
                                .notice
                                .map_or_else(String::new, |n| format!(" — {n}"))
                        );
                    }
                }
            }
            Action::Done => {
                self.status = format!("{} crafted.", job.spell_name);
                // Re-read at once: the new scroll and the spent components
                // both change what the rows say.
                self.ticks = BUSY_TICKS;
            }
            Action::Failed(why) => {
                self.status = why;
                self.queue.clear();
            }
        }
    }

    fn handle_command(&mut self, host: &mut Host<'_>, argument: &str) {
        if argument.eq_ignore_ascii_case("show") {
            if let Some(panel) = self.panel.as_ref() {
                panel.show(host, true);
            }
            return;
        }
        if !self.built {
            self.build(host);
        }
        for line in self.report(argument.eq_ignore_ascii_case("full")) {
            host.chat_post(&line);
        }
    }

    /// What the recipe table and the spell table agree on.
    fn report(&self, full: bool) -> Vec<String> {
        let mut out = Vec::new();
        if self.recipes.is_empty() {
            out.push(
                "Spell Research: the recipe table matched no spell on this server.".to_owned(),
            );
            return out;
        }
        out.push(format!(
            "Spell Research: {} researchable incantations ({} recipes in the table).",
            self.recipes.len(),
            formulas::FORMULAS.len(),
        ));

        let mut by_school: BTreeMap<u32, usize> = BTreeMap::new();
        for recipe in &self.recipes {
            *by_school.entry(recipe.school).or_default() += 1;
        }
        for (school, count) in &by_school {
            out.push(format!("    school {school}: {count}"));
        }

        if full {
            for recipe in &self.recipes {
                out.push(format!(
                    "  {} [{}] {} = {} + {} + {}",
                    recipe.spell_id,
                    recipe.school,
                    recipe.name,
                    recipe.quill,
                    recipe.ink,
                    recipe.glyph
                ));
            }
        } else {
            out.push("Spell Research: say `/spellresearch full` for every spell.".to_owned());
        }
        out
    }

    /// Switch school tabs. Queued spells from the old tab are dropped —
    /// the queue only ever concerns what is on screen — and a running job
    /// whose spell left the tab stops at its next combine.
    fn set_tab(&mut self, tab: Tab) {
        if self.tab == tab {
            return;
        }
        self.tab = tab;
        let dropped = self.queue.len();
        self.queue.clear();
        if dropped > 0 {
            self.status = format!(
                "{} tab. Dropped {dropped} queued spell(s) from the old tab.",
                tab.label()
            );
        }
    }

    fn handle_ui(&mut self, host: &mut Host<'_>, handler: &str, value: ActionValue) {
        let row = match &value {
            ActionValue::Int(index) => usize::try_from(*index).ok(),
            _ => None,
        };
        match handler {
            "Craft" => {
                if let Some(id) = row.and_then(|index| self.rows.spell_ids.get(index).copied()) {
                    if !self.queue.contains(&id) {
                        self.queue.push_back(id);
                        self.status = format!("Queued. {} waiting.", self.queue.len());
                    }
                    self.ticks = BUSY_TICKS;
                }
            }
            "CraftAll" => {
                let ready = self.rows.ready();
                let count = ready.len();
                for id in ready {
                    if !self.queue.contains(&id) {
                        self.queue.push_back(id);
                    }
                }
                self.status = format!("Queued {count} spell(s) from {}.", self.tab.label());
                self.ticks = BUSY_TICKS;
            }
            "Refresh" => {
                self.built = false;
                self.build(host);
            }
            "TabAll" => self.set_tab(Tab::All),
            "TabCreature" => self.set_tab(Tab::Creature),
            "TabItem" => self.set_tab(Tab::Item),
            "TabLife" => self.set_tab(Tab::Life),
            "TabWarVoid" => self.set_tab(Tab::WarVoid),
            "ToggleUnknown" => self.unknown_only = !self.unknown_only,
            "ToggleReady" => self.ready_only = !self.ready_only,
            "Select" => {
                if let Some(recipe) = row.and_then(|index| {
                    self.rows
                        .spell_ids
                        .get(index)
                        .and_then(|id| self.recipes.iter().find(|r| &r.spell_id == id))
                }) {
                    self.status = describe(recipe);
                }
            }
            _ => {}
        }
        if matches!(
            handler,
            "ToggleUnknown"
                | "ToggleReady"
                | "Select"
                | "TabAll"
                | "TabCreature"
                | "TabItem"
                | "TabLife"
                | "TabWarVoid"
        ) {
            self.refresh(host);
        }
    }
}

/// One recipe in words, for the status line.
fn describe(recipe: &Recipe) -> String {
    let ink = if matches!(recipe.ink, "alacritous" | "parabolic") {
        format!("{} ink", crate::component::title(recipe.ink))
    } else {
        format!("ink of {}", recipe.ink)
    };
    format!(
        "{}: quill of {}, {ink}, glyph of {} — and a mana scarab to infuse",
        recipe.name, recipe.quill, recipe.glyph
    )
}

openac_plugin::export_plugin!(SpellResearch);

#[cfg(test)]
mod tests {
    use super::*;

    /// Start the plugin the way the client does, through the SDK's fake host,
    /// which applies the same verb and panel rules the real one does. This is
    /// what catches a `/spell-research`-style mistake: the client refuses the
    /// whole plugin, not just the command, so nothing loads and the window
    /// never appears.
    #[test]
    fn the_plugin_starts_and_registers_its_window() {
        let mut host = openac_plugin::testing::FakeHost::new("evantaylor.spell-research");
        let mut plugin = SpellResearch::default();
        host.enable(&mut plugin)
            .expect("the plugin starts: its verb and panel must satisfy the host");
        assert!(plugin.panel.is_some(), "the window was not registered");
    }

    #[test]
    fn the_report_survives_an_empty_spell_table() {
        let plugin = SpellResearch::default();
        let lines = plugin.report(false);
        assert_eq!(lines.len(), 1);
        assert!(lines[0].contains("matched no spell"), "{}", lines[0]);
    }

    /// The known check must ask every id a spell is listed under, not just
    /// the row's own: the server carries some spells (Nullify Item Magic,
    /// the weapon auras) under several ids, and which one a character
    /// learned is not the plugin's choice. A recipe built from all of them
    /// reports known when any is.
    #[test]
    fn a_recipe_is_known_when_any_of_its_duplicate_ids_is() {
        let formula = formulas::FORMULAS
            .iter()
            .find(|f| f.spell == "Incantation of Nullify Item Magic")
            .expect("the table carries Nullify Item Magic");
        // The server lists this spell under 4342, 4343 and 4344.
        let entries: Vec<openac_plugin::magic::SpellInfo> = [4342u32, 4343, 4344]
            .iter()
            .map(|id| openac_plugin::magic::SpellInfo {
                spell_id: *id,
                name: formula.spell.to_owned(),
                ..Default::default()
            })
            .collect();
        let refs: Vec<&openac_plugin::magic::SpellInfo> = entries.iter().collect();
        let recipe = Recipe::new(formula, refs).expect("three entries");
        assert_eq!(recipe.ids, vec![4342, 4343, 4344]);
        // Knowing only the highest duplicate still counts as known — this
        // is what refresh() asks, and what it got wrong when it checked
        // only recipe.spell_id.
        assert!(recipe.is_known(&|id| id == 4344));
    }
}
