//! Cross gui commands

use crate::state::State;

#[derive(Debug, Clone, Copy)]
pub enum Messages {
    Increment,
    Decrement,
}

/// Run messages
pub fn call(state: &mut State, message: Messages) {
    match message {
        Messages::Increment => {
            state.count += 1;
        }
        Messages::Decrement => {
            state.count -= 1;
        }
    }
}
