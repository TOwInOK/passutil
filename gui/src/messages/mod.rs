//! Cross gui commands

use crate::{state::State, ui::filepicker::file_pick};

#[derive(Debug, Clone)]
pub enum Message {
    PickAvatar,
}

/// Run messages
pub fn call(state: &mut State, message: Message) {
    match message {
        Message::PickAvatar => state.avatar_path = file_pick(),
    }
}
