use std::path::PathBuf;

use crate::state::{Message, State};
use iced::alignment::Horizontal;
use iced::widget::{button, column, text};
use rfd::FileDialog;

// Message
pub fn file_pick() -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("img", &["png"])
        .set_directory("./")
        .pick_file()
}

pub fn folder_pick() -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("img", &["png"])
        .set_directory("./")
        .pick_folder()
}

/// Component
pub fn file_picker(state: &State) -> iced::Element<Message> {
    column![
        text(if let Some(avatar) = &state.avatar_path {
            avatar.to_str().unwrap_or("Invalid path")
        } else {
            "Pick something"
        })
        .size(20),
        button("Pick").on_press(Message::PickAvatar)
    ]
    .align_x(Horizontal::Center)
    .into()
}
