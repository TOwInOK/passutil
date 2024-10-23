use filepicker::file_picker;
use iced::alignment::Horizontal::{self};
use iced::widget::{column, container, Container};

use crate::state::Message;

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
    let app = column![file_picker(state)];
    container(app).align_x(Horizontal::Center)
}
