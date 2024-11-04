//! Cross gui commands

use crate::{
    state::{Message, State},
    ui::filepicker::{file_pick, folder_pick},
};

/// Run messages
pub fn call(state: &mut State, message: Message) {
    match message {
        Message::PickAvatar => state.avatar_path = file_pick(),
        Message::PickFriends => state.current_friend_folder_path = folder_pick(),
        Message::ForceAssetsReload => todo!(),
        Message::SelectBackground => todo!(),
        Message::SelectBadge => todo!(),
        Message::SelectFrame => todo!(),
        Message::SelectBanner => todo!(),
        Message::SwitchWindowPin => todo!(),
        Message::SwitchWindowBanner => todo!(),
        Message::SwitchWindowBackground => todo!(),
        Message::SwitchWindowFrame => todo!(),
        Message::SwitchWindowAbout => todo!(),
    }
}
