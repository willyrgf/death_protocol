use crate::healthchecks::telegram::dialogue::{states::checked::CheckedState, Dialogue};
use teloxide::prelude::*;

pub struct StartState;

#[teloxide(subtransition)]
async fn start(
    _state: StartState,
    cx: TransitionIn<AutoSend<Bot>>,
    _ans: String,
) -> TransitionOut<Dialogue> {
    cx.answer("start state").await?;
    next(CheckedState)
}
