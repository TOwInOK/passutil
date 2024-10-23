//! Cross gui commands

use crate::{
    state::{Message, State},
    ui::filepicker::file_pick,
};

/// Run messages
pub fn call(state: &mut State, message: Message) {
    match message {
        Message::PickAvatar => state.avatar_path = file_pick(),
    }
}
