use crate::messages::Message;
use filepicker::file_picker;
use iced::alignment::Horizontal::{self};
use iced::widget::{button, column, container, text, Container};

/// File picker for user image
pub mod filepicker;
/// footer
pub mod footer;
/// header
pub mod header;
/// Image board of images by path
pub mod imagegrid;
/// Selector for choose path
pub mod selector;

/// Render app
pub fn render(state: &crate::state::State) -> Container<Message> {
    container(file_picker(state)).align_x(Horizontal::Center))
}
