use crate::healthchecks::telegram::dialogue::Dialogue;
use teloxide::prelude::*;

pub struct CheckedState;

#[teloxide(subtransition)]
async fn checked(
    _state: CheckedState,
    cx: TransitionIn<AutoSend<Bot>>,
    _ans: String,
) -> TransitionOut<Dialogue> {
    cx.answer("life checked.").await?;
    exit()
}
