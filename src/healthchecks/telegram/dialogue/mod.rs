use self::states::{checked::CheckedState, start::StartState};
use derive_more::From;
use teloxide::prelude::*;

pub mod states;

#[derive(Transition, From, Clone)]
pub enum Dialogue {
    Start(StartState),
    Checked(CheckedState),
}

impl Default for Dialogue {
    fn default() -> Self {
        Self::Start(StartState)
    }
}
