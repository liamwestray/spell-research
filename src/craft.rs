//! The three combines that make a level VIII scroll, as a state machine.
//!
//! ```text
//! quill + mana scarab  ->  infused quill
//! infused quill + ink  ->  the combination
//! combination + glyph ->  the scroll
//! ```
//!
//! Each combine consumes its two items and produces a new one, whose client
//! id the plugin cannot know in advance — the product is not in the spell
//! table and has no id until the server creates it. So the job snapshots the
//! pack before each combine and reads the product back out as the object that
//! appeared. That is also what makes a failure safe to report: the wiki is
//! clear that a failed combine consumes nothing, so a job that stops leaves
//! the components where they were.
//!
//! The machine is pure — it is handed a [`Snapshot`] of the pack and answers
//! with an [`Action`] — so the whole sequence, including the awkward cases,
//! is tested without a running client.

use crate::recipe::Requirement;

/// What the pack looks like at one moment.
#[derive(Debug, Clone, Default)]
pub struct Snapshot {
    /// Every object the character owns.
    pub objects: Vec<u32>,
    /// How many uses have finished. `Host::item_state().last_completion.revision`.
    pub revision: i64,
    /// The server's error on the last finished use, 0 for none.
    pub error: u32,
}

/// What the job asks the caller to do next.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// Nothing to do yet: the combine in flight has not been answered.
    Waiting,
    /// Use `base` on `target`, remembering the current revision.
    Apply { base: u32, target: u32 },
    /// The scroll was made.
    Done,
    /// The job stopped, with the reason for the player.
    Failed(String),
}

/// One combine left: what to use the base on.
#[derive(Debug, Clone)]
pub struct Step {
    /// What the item is, for finding it in the pack.
    pub requirement: Requirement,
    /// Its name, for the message when it is missing.
    pub label: String,
}

/// One scroll being made, step by step.
#[derive(Debug, Clone)]
pub struct Job {
    /// The spell being made, so the caller can stop the job when its row
    /// is no longer on screen.
    pub spell_id: u32,
    /// Its name, for the messages the player sees.
    pub spell_name: String,
    /// The combines left.
    queue: Vec<Step>,
    /// The item the next combine uses as its base.
    base: u32,
    /// Set while a combine waits for the server.
    in_flight: Option<InFlight>,
}

#[derive(Debug, Clone)]
struct InFlight {
    /// The revision to beat.
    after: i64,
    /// The pack before the combine, to find the product in.
    before: Vec<u32>,
    /// The combine this is, so a failure names it and an unsent one goes back.
    step: Step,
}

impl Job {
    /// A job that starts from `base` (the quill) and has `queue` left.
    #[must_use]
    pub fn new(spell_id: u32, spell_name: String, base: u32, queue: Vec<Step>) -> Self {
        Self {
            spell_id,
            spell_name,
            queue,
            base,
            in_flight: None,
        }
    }

    /// Whether a combine has been sent and awaits its answer. A combine
    /// already sent cannot be recalled, so a job that is in flight is left
    /// to settle before it stops.
    #[must_use]
    pub fn is_in_flight(&self) -> bool {
        self.in_flight.is_some()
    }

    /// The next thing to do, given how the pack looks now.
    ///
    /// `find` answers what to use the base on next; it is only asked when the
    /// job is not already waiting on the server.
    pub fn drive(
        &mut self,
        now: &Snapshot,
        mut find: impl FnMut(&Requirement) -> Option<u32>,
    ) -> Action {
        if let Some(flight) = self.in_flight.clone() {
            if now.revision <= flight.after {
                return Action::Waiting;
            }
            self.in_flight = None;
            if now.error != 0 {
                return Action::Failed(format!(
                    "{}: the server refused the combine (error {})",
                    self.spell_name, now.error
                ));
            }
            if self.queue.is_empty() {
                return Action::Done;
            }
            // The product is the object that was not there before.
            match now.objects.iter().find(|id| !flight.before.contains(id)) {
                Some(product) => self.base = *product,
                None => {
                    return Action::Failed(format!(
                        "{}: the {} produced nothing new in the pack",
                        self.spell_name, flight.step.label
                    ));
                }
            }
        }

        let Some(step) = self.queue.first().cloned() else {
            return Action::Done;
        };
        let Some(target) = find(&step.requirement) else {
            return Action::Failed(format!(
                "{}: no {} in the pack",
                self.spell_name, step.label
            ));
        };
        self.queue.remove(0);
        self.in_flight = Some(InFlight {
            after: now.revision,
            before: now.objects.clone(),
            step: step.clone(),
        });
        Action::Apply {
            base: self.base,
            target,
        }
    }

    /// Put the combine back: it was never sent, so nothing was consumed and
    /// the job can try again when the pack is free.
    pub fn undo(&mut self) {
        if let Some(flight) = self.in_flight.take() {
            self.queue.insert(0, flight.step);
        }
    }

    /// Combines left, including the one in flight.
    #[must_use]
    pub fn remaining(&self) -> usize {
        self.queue.len() + usize::from(self.in_flight.is_some())
    }
}

/// The combines a recipe needs after its quill: scarab, ink, glyph.
///
/// A role the recipe does not name is skipped, so a spell the model cannot
/// fully describe stops early rather than combining the wrong things.
#[must_use]
pub fn steps_for(recipe: &crate::recipe::Recipe) -> Vec<Step> {
    recipe
        .requirements()
        .into_iter()
        .filter(|want| want.role != crate::component::Role::Quill)
        .map(|requirement| Step {
            label: requirement
                .variant
                .clone()
                .unwrap_or_else(|| requirement.role.label().to_owned()),
            requirement,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::component::Role;

    fn step(role: Role, variant: &str) -> Step {
        Step {
            requirement: Requirement {
                role,
                variant: Some(variant.to_owned()),
            },
            label: variant.to_owned(),
        }
    }

    /// A job with the quill in hand and three combines left.
    fn job() -> Job {
        Job::new(
            1,
            "Test Spell".to_owned(),
            100,
            vec![
                step(Role::Scarab, "mana"),
                step(Role::Ink, "conveyance"),
                step(Role::Glyph, "strength"),
            ],
        )
    }

    fn pack(objects: &[u32], revision: i64, error: u32) -> Snapshot {
        Snapshot {
            objects: objects.to_vec(),
            revision,
            error,
        }
    }

    /// Every requirement finds the object named by its variant.
    fn finder<'a>(table: &'a [(&'a str, u32)]) -> impl FnMut(&Requirement) -> Option<u32> + 'a {
        move |want: &Requirement| {
            want.variant.as_ref().and_then(|variant| {
                table
                    .iter()
                    .find(|(name, _)| *name == variant.as_str())
                    .map(|(_, id)| *id)
            })
        }
    }

    #[test]
    fn the_first_move_uses_the_quill_on_the_scarab() {
        let mut job = job();
        let found = [("mana", 200u32), ("conveyance", 300), ("strength", 400)];
        let action = job.drive(&pack(&[100, 200, 300, 400], 0, 0), finder(&found));
        assert_eq!(
            action,
            Action::Apply {
                base: 100,
                target: 200
            }
        );
        assert_eq!(job.remaining(), 3, "two queued and one in flight");
    }

    #[test]
    fn a_sent_combine_marks_the_job_in_flight() {
        // The caller stops a job whose spell left the window, but only once
        // any combine it already sent has settled.
        let mut job = job();
        assert!(!job.is_in_flight(), "nothing sent yet");
        let found = [("mana", 200u32), ("conveyance", 300), ("strength", 400)];
        job.drive(&pack(&[100, 200, 300, 400], 0, 0), finder(&found));
        assert!(job.is_in_flight(), "the combine awaits its answer");
    }

    #[test]
    fn it_waits_rather_than_double_applying() {
        let mut job = job();
        let found = [("mana", 200u32), ("conveyance", 300), ("strength", 400)];
        job.drive(&pack(&[100, 200, 300, 400], 0, 0), finder(&found));
        // The same revision: the server has not answered.
        assert_eq!(
            job.drive(&pack(&[100, 300, 400], 0, 0), finder(&found)),
            Action::Waiting
        );
    }

    #[test]
    fn the_second_step_uses_the_product_of_the_first() {
        let mut job = job();
        let found = [("mana", 200u32), ("conveyance", 300), ("strength", 400)];
        job.drive(&pack(&[100, 200, 300, 400], 0, 0), finder(&found));
        // The quill and scarab are gone; 500 is the infused quill.
        let action = job.drive(&pack(&[300, 400, 500], 1, 0), finder(&found));
        assert_eq!(
            action,
            Action::Apply {
                base: 500,
                target: 300
            }
        );
    }

    #[test]
    fn the_whole_chain_finishes_in_three_combines() {
        let mut job = job();
        let found = [("mana", 200u32), ("conveyance", 300), ("strength", 400)];
        let steps = [
            (
                pack(&[100, 200, 300, 400], 0, 0),
                Action::Apply {
                    base: 100,
                    target: 200,
                },
            ),
            (
                pack(&[300, 400, 500], 1, 0),
                Action::Apply {
                    base: 500,
                    target: 300,
                },
            ),
            (
                pack(&[400, 600], 2, 0),
                Action::Apply {
                    base: 600,
                    target: 400,
                },
            ),
            (pack(&[700], 3, 0), Action::Done),
        ];
        for (snapshot, want) in steps {
            assert_eq!(job.drive(&snapshot, finder(&found)), want);
        }
    }

    #[test]
    fn a_refused_combine_fails_the_job_with_the_server_error() {
        let mut job = job();
        let found = [("mana", 200u32), ("conveyance", 300), ("strength", 400)];
        job.drive(&pack(&[100, 200, 300, 400], 0, 0), finder(&found));
        let action = job.drive(&pack(&[100, 200, 300, 400], 1, 42), finder(&found));
        match action {
            Action::Failed(why) => {
                assert!(why.contains("Test Spell"), "{why}");
                assert!(why.contains("42"), "{why}");
            }
            other => panic!("expected a failure, got {other:?}"),
        }
    }

    #[test]
    fn a_combine_that_produces_nothing_fails_rather_than_reusing_the_quill() {
        let mut job = job();
        let found = [("mana", 200u32), ("conveyance", 300), ("strength", 400)];
        job.drive(&pack(&[100, 200, 300, 400], 0, 0), finder(&found));
        // Answered, no error, and no new object: the base would otherwise
        // stay the quill and the next combine would be nonsense.
        let action = job.drive(&pack(&[100, 200, 300, 400], 1, 0), finder(&found));
        match action {
            Action::Failed(why) => assert!(why.contains("produced nothing"), "{why}"),
            other => panic!("expected a failure, got {other:?}"),
        }
    }

    #[test]
    fn a_missing_component_stops_before_anything_is_consumed() {
        let mut job = job();
        // No scarab in the pack.
        let found = [("conveyance", 300u32), ("strength", 400)];
        let action = job.drive(&pack(&[100, 300, 400], 0, 0), finder(&found));
        match action {
            Action::Failed(why) => assert!(why.contains("no mana"), "{why}"),
            other => panic!("expected a failure, got {other:?}"),
        }
    }

    #[test]
    fn a_job_with_nothing_left_is_done() {
        let mut job = Job::new(1, "Empty".to_owned(), 100, vec![]);
        let found: [(&str, u32); 0] = [];
        assert_eq!(job.drive(&pack(&[100], 0, 0), finder(&found)), Action::Done);
    }
}
