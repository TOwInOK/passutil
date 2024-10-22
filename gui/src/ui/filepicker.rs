use std::path::PathBuf;

use crate::{messages::Message, state::State};
use iced::alignment::Horizontal;
use iced::widget::{button, column, text};
use rfd::FileDialog;

// Message
pub fn file_pick() -> Option<PathBuf> {
    // if you want directly add path: &Type into ()
    // let path = path
    //     .get_file("avatar")?
    //     .ok_or(Error::NotFound("avatar.png".to_string()))?;
    FileDialog::new()
        .add_filter("img", &["png"])
        .set_directory("/")
        .pick_file()
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
